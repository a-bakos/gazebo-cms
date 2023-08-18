use crate::traits::RowTransformer;
use gazebo_core_common::entry::gb_log::GB_Log;
use sqlx::postgres::PgRow;

impl RowTransformer<PgRow> for GB_Log {
    type Output = GB_Log;

    fn transform(row: &PgRow) -> Self::Output {
        todo!()
    }
}
