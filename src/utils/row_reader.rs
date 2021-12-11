use sqlx::Row;

pub struct RowReader<D: sqlx::Database> {
    index: usize,
    row: D::Row,
}

impl<D: sqlx::Database> RowReader<D> {
    pub fn from_row(row: <D as sqlx::Database>::Row) -> Self {
        Self { index: 0, row }
    }

    #[inline]
    pub fn read_next<'r, T>(&'r mut self) -> T
    where
        T: sqlx::Decode<'r, D> + sqlx::Type<D>,
        usize: sqlx::ColumnIndex<<D as sqlx::Database>::Row>,
    {
        let result = self.row.try_get::<T, _>(self.index).unwrap();
        self.index += 1;
        result
    }
}
