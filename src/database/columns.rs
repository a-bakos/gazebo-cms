use std::collections::HashMap;

// TODO later: think about tables

pub const COL_INDEX_ID: usize = 0;
pub const COL_INDEX_ID_AUTHOR: usize = 1;
#[allow(dead_code)]
pub const COL_INDEX_PARENT: usize = 2;
pub const COL_INDEX_DATE_PUBLISH: usize = 3;
pub const COL_INDEX_DATE_MODIFIED: usize = 4;
pub const COL_INDEX_SLUG: usize = 5;
#[allow(dead_code)]
pub const COL_INDEX_POST_TYPE: usize = 6;
pub const COL_INDEX_TITLE: usize = 7;
#[allow(dead_code)]
pub const COL_INDEX_EXCERPT: usize = 8;
#[allow(dead_code)]
pub const COL_INDEX_CONTENT: usize = 9;
#[allow(dead_code)]
pub const COL_INDEX_PASSWORD: usize = 10;

#[allow(dead_code)]
pub const COLUMNS: [(&str, usize); 11] = [
    ("ID", COL_INDEX_ID),
    ("ID_AUTHOR", COL_INDEX_ID_AUTHOR),
    ("PARENT", COL_INDEX_PARENT),
    ("DATE_PUBLISH", COL_INDEX_DATE_PUBLISH),
    ("DATE_MODIFIED", COL_INDEX_DATE_MODIFIED),
    ("SLUG", COL_INDEX_SLUG),
    ("POST_TYPE", COL_INDEX_POST_TYPE),
    ("TITLE", COL_INDEX_TITLE),
    ("EXCERPT", COL_INDEX_EXCERPT),
    ("CONTENT", COL_INDEX_CONTENT),
    ("PASSWORD", COL_INDEX_PASSWORD),
];

#[allow(dead_code)]
pub fn get_columns() -> HashMap<String, usize> {
    let mut columns: HashMap<String, usize> = HashMap::new();
    for (col_name, col_index) in COLUMNS.iter() {
        columns.insert(col_name.to_string(), *col_index);
    }
    columns
}
