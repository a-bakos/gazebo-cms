pub trait RowTransformer<Row> {
    type Output;
    fn transform(row: &Row) -> Self::Output;
}
