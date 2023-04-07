use crate::database::db::DB_Table;
use std::collections::HashMap;

// TODO later: think about tables

pub const COL_INDEX_POST_ID: usize = 0;
pub const COL_INDEX_POST_ID_AUTHOR: usize = 1;
#[allow(dead_code)]
pub const COL_INDEX_POST_PARENT: usize = 2;
pub const COL_INDEX_POST_DATE_PUBLISH: usize = 3;
pub const COL_INDEX_POST_DATE_MODIFIED: usize = 4;
pub const COL_INDEX_POST_SLUG: usize = 5;
#[allow(dead_code)]
pub const COL_INDEX_POST_TYPE: usize = 6;
pub const COL_INDEX_POST_TITLE: usize = 7;
#[allow(dead_code)]
pub const COL_INDEX_POST_EXCERPT: usize = 8;
#[allow(dead_code)]
pub const COL_INDEX_POST_CONTENT: usize = 9;
#[allow(dead_code)]
pub const COL_INDEX_POST_PASSWORD: usize = 10;

#[allow(dead_code)]
pub const COLUMNS_POSTS: [(&str, usize); 11] = [
    ("ID", COL_INDEX_POST_ID),
    ("ID_AUTHOR", COL_INDEX_POST_ID_AUTHOR),
    ("PARENT", COL_INDEX_POST_PARENT),
    ("DATE_PUBLISH", COL_INDEX_POST_DATE_PUBLISH),
    ("DATE_MODIFIED", COL_INDEX_POST_DATE_MODIFIED),
    ("SLUG", COL_INDEX_POST_SLUG),
    ("POST_TYPE", COL_INDEX_POST_TYPE),
    ("TITLE", COL_INDEX_POST_TITLE),
    ("EXCERPT", COL_INDEX_POST_EXCERPT),
    ("CONTENT", COL_INDEX_POST_CONTENT),
    ("PASSWORD", COL_INDEX_POST_PASSWORD),
];

#[allow(dead_code)]
pub fn get_columns(table: DB_Table) -> HashMap<String, usize> {
    let mut columns: HashMap<String, usize> = HashMap::new();
    match table {
        DB_Table::Posts => {
            for (col_name, col_index) in COLUMNS_POSTS.iter() {
                columns.insert(col_name.to_string(), *col_index);
            }
        }
        DB_Table::Users => {
            todo!()
        }
    }
    columns
}
