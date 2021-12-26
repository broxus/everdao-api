use std::time::Duration;

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

#[macro_export]
macro_rules! once {
    ($ty:path, || $expr:expr) => {{
        static ONCE: once_cell::race::OnceBox<$ty> = once_cell::race::OnceBox::new();
        ONCE.get_or_init(|| Box::new($expr))
    }};
}

pub async fn poll_run_local(
    transaction_producer: &ton_consumer::TransactionProducer,
    contract_address: &ton_block::MsgAddressInt,
    function: &ton_abi::Function,
    input: &[ton_abi::Token],
    timeout: u64,
) -> anyhow::Result<nekoton_abi::ExecutionOutput> {
    let now = std::time::Instant::now();
    loop {
        if let Some(function_output) = transaction_producer
            .run_local(contract_address, function, input)
            .await?
        {
            break Ok(function_output);
        }

        if now.elapsed().as_secs() > timeout {
            break Err(anyhow::Error::msg("none function output"));
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
