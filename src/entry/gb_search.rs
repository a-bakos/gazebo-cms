use crate::traits::RowTransformer;
use gazebo_core_common::entry::gb_search::GB_Search;
use sqlx::postgres::PgRow;

impl RowTransformer<PgRow> for GB_Search {
    type Output = GB_Search;

    fn transform(row: &PgRow) -> Self::Output {
        todo!()
    }
}
