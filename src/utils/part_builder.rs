use itertools::Itertools;
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Encode, Postgres, Type};

pub struct OwnedPartBuilder {
    args: PgArguments,
    data: String,
}

impl OwnedPartBuilder {
    pub fn new() -> Self {
        Self {
            args: PgArguments::default(),
            data: String::default(),
        }
    }

    pub fn reserve_args(mut self, args: usize, bytes: usize) -> Self {
        self.args.reserve(args, bytes);
        self
    }

    pub fn starts_with<S: std::fmt::Display>(mut self, data: S) -> Self {
        self.data = data.to_string();
        self
    }

    pub fn push<S: std::fmt::Display>(&mut self, part: S) -> &mut Self {
        self.data = format!("{} {}", self.data, part);
        self
    }

    pub fn push_arg<P>(&mut self, param: P) -> &mut Self
    where
        for<'q> P: Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        self.args.add(param);
        self
    }

    pub fn push_with_arg<S, P>(&mut self, part: S, param: P) -> &mut Self
    where
        S: std::fmt::Display,
        for<'q> P: Encode<'q, Postgres> + Type<Postgres> + Send,
    {
        self.push(part);
        self.push_arg(param);
        self
    }

    pub fn push_part<T: QueryPart>(&mut self, part: T) -> &mut Self {
        part.write_into(self);
        self
    }

    pub fn split(self) -> (String, PgArguments) {
        (self.data, self.args)
    }
}

impl Default for OwnedPartBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BytesWrapper<const N: usize>(pub [u8; N]);

impl<'a, const N: usize> Encode<'a, Postgres> for BytesWrapper<N> {
    fn encode(self, buf: &mut <Postgres as HasArguments<'a>>::ArgumentBuffer) -> IsNull
    where
        Self: Sized,
    {
        <&[u8] as Encode<'a, Postgres>>::encode(self.0.as_ref(), buf)
    }

    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'a>>::ArgumentBuffer) -> IsNull {
        <&[u8] as Encode<'a, Postgres>>::encode_by_ref(&self.0.as_ref(), buf)
    }

    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        <&[u8] as Encode<'a, Postgres>>::produces(&self.0.as_ref())
    }

    fn size_hint(&self) -> usize {
        <&[u8] as Encode<'a, Postgres>>::size_hint(&self.0.as_ref())
    }
}

impl<const N: usize> Type<Postgres> for BytesWrapper<N> {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <&[u8] as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        <&[u8] as Type<Postgres>>::compatible(ty)
    }
}

pub trait QueryPart: Send + Sync {
    fn write_into(self, builder: &mut OwnedPartBuilder);
}

impl<T> QueryPart for (String, T)
where
    for<'q> T: Encode<'q, Postgres> + Type<Postgres> + Send + Sync,
{
    fn write_into(self, builder: &mut OwnedPartBuilder) {
        builder.push_with_arg(self.0, self.1);
    }
}

pub struct Negate<T>(pub T);

impl<T> QueryPart for Negate<T>
where
    T: QueryPart,
{
    fn write_into(self, builder: &mut OwnedPartBuilder) {
        builder.push("NOT (").push_part(self.0).push(")");
    }
}

pub struct BindArgs<T>(pub T);

macro_rules! impl_multiple_args {
    ($($n:tt: $ty:ident),+) => {
        impl<$($ty),+> QueryPart for BindArgs<($($ty,)+)>
        where
            $(for <'q> $ty: Encode<'q, Postgres> + Type<Postgres> + Send + Sync),+
        {
            fn write_into(self, builder: &mut OwnedPartBuilder) {
                $(builder.push_arg(&self.0.$n);)?
            }
        }
    }
}

impl_multiple_args!(0: T0);
impl_multiple_args!(0: T0, 1: T1);

pub enum CustomBuildType {
    Int(i64),
    Bool(bool),
}

pub struct CustomBuild<I>(pub String, pub I);

impl<I> QueryPart for CustomBuild<I>
where
    I: IntoIterator<Item = CustomBuildType> + Send + Sync,
{
    fn write_into(self, builder: &mut OwnedPartBuilder) {
        for param in self.1.into_iter() {
            match param {
                CustomBuildType::Int(param) => builder.push_arg(param),
                CustomBuildType::Bool(param) => builder.push_arg(param),
            };
        }

        builder.push(self.0);
    }
}

pub struct FieldRange<I>(pub &'static str, pub I);

impl<I, T> QueryPart for FieldRange<I>
where
    I: IntoIterator<Item = T> + Send + Sync,
    T: FieldRangeItem,
    BindArgs<T::Args>: QueryPart,
{
    fn write_into(self, builder: &mut OwnedPartBuilder) {
        let mut iter = self.1.into_iter().peekable();
        if iter.peek().is_none() {
            builder.push("FALSE");
            return;
        }

        let range = iter
            .map(|item| {
                builder.push_part(BindArgs(item.into_args()));
                T::ITEM_PATTERN
            })
            .join(",");

        builder.push(format!("({} IN ({}))", self.0, range));
    }
}

pub trait FieldRangeItem {
    type Args;

    const ITEM_PATTERN: &'static str;

    fn into_args(self) -> Self::Args;
}

pub struct WhereAndConditions<T>(pub T);

macro_rules! impl_where_condition_tuple {
    ($n_first:tt: $ty_first:ident, $($n:tt: $ty:ident),+) => {
        impl_where_condition_tuple!(@repeat { $n_first: $ty_first } $($n: $ty),+);
    };

    (@repeat { $($n_before:tt: $ty_before:ident),+ } $n:tt: $ty:ident $(, $n_after:tt: $ty_after:ident)*) => {
        impl_where_condition_tuple!(@impl $($n_before: $ty_before),+, $n: $ty);

        impl_where_condition_tuple!(@repeat { $($n_before: $ty_before),+, $n: $ty } $($n_after: $ty_after),*);
    };

    (@repeat { $($n_before:tt: $ty_after:ident),+ }) => {};

    (@impl $($n:tt: $ty:ident),+) => {
        impl<$($ty),+> QueryPart for WhereAndConditions<($(Option<$ty>),+)>
        where
            $($ty: QueryPart),+
        {
            fn write_into(self, builder: &mut OwnedPartBuilder) {
                let mut first = true;
                $(if let Some(condition) = self.0.$n {
                    if ::std::mem::take(&mut first) {
                        builder.push("WHERE");
                    } else {
                        builder.push("AND");
                    }
                    builder.push_part(condition);
                })?
            }
        }
    }
}

impl_where_condition_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11
);
