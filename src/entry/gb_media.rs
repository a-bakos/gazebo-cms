use crate::traits::RowTransformer;
use gazebo_core_common::entry::gb_media::GB_Media;
use sqlx::postgres::PgRow;

impl RowTransformer<PgRow> for GB_Media {
    type Output = GB_Media;

    fn transform(row: &PgRow) -> Self::Output {
        todo!()
    }
}
