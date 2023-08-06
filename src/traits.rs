pub trait RowTransformer<Row> {
    type Output;
    fn transform(row: &Row) -> Self::Output;
}

// IDEA
pub trait IntoSqlQuery<GB_Type> {
    fn sql_insert(row: &GB_Type) -> String;
}
