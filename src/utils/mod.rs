pub use self::part_builder::*;
pub use self::row_reader::*;

mod part_builder;
mod row_reader;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Paginated<T> {
    pub limit: u32,
    pub offset: u32,
    pub data: T,
}

pub trait AsPaginated: Sized {
    fn paginated(self, limit: u32, offset: u32) -> Paginated<Self>;
}

impl<T> AsPaginated for T {
    fn paginated(self, limit: u32, offset: u32) -> Paginated<Self> {
        Paginated {
            limit,
            offset,
            data: self,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ordered<T, O> {
    pub filters: T,
    pub ordering: Option<O>,
}

pub trait AsOrdered<O>: Sized {
    fn ordered(self, ordering: Option<O>) -> Ordered<Self, O>;
}

impl<T, O> AsOrdered<O> for T {
    fn ordered(self, ordering: Option<O>) -> Ordered<Self, O> {
        Ordered {
            filters: self,
            ordering,
        }
    }
}
