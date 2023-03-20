use std::collections::HashMap;

pub const COL_INDEX_ID: usize = 0;
pub const COL_INDEX_ID_AUTHOR: usize = 1;
pub const COL_INDEX_PARENT: usize = 2;
pub const COL_INDEX_DATE_PUBLISH: usize = 3;
pub const COL_INDEX_DATE_MODIFIED: usize = 4;
pub const COL_INDEX_SLUG: usize = 5;
pub const COL_INDEX_POST_TYPE: usize = 6;
pub const COL_INDEX_TITLE: usize = 7;
pub const COL_INDEX_EXCERPT: usize = 8;
pub const COL_INDEX_CONTENT: usize = 9;
pub const COL_INDEX_PASSWORD: usize = 10;

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

pub fn get_columns() -> HashMap<String, usize> {
    let mut columns: HashMap<String, usize> = HashMap::new();
    columns.insert(COLUMNS[0].0.to_string(), COLUMNS[0].1);
    columns.insert(COLUMNS[1].0.to_string(), COLUMNS[1].1);
    columns.insert(COLUMNS[2].0.to_string(), COLUMNS[2].1);
    columns.insert(COLUMNS[3].0.to_string(), COLUMNS[3].1);
    columns.insert(COLUMNS[4].0.to_string(), COLUMNS[4].1);
    columns.insert(COLUMNS[5].0.to_string(), COLUMNS[5].1);
    columns.insert(COLUMNS[6].0.to_string(), COLUMNS[6].1);
    columns.insert(COLUMNS[7].0.to_string(), COLUMNS[7].1);
    columns.insert(COLUMNS[8].0.to_string(), COLUMNS[8].1);
    columns.insert(COLUMNS[9].0.to_string(), COLUMNS[9].1);
    columns.insert(COLUMNS[10].0.to_string(), COLUMNS[10].1);
    columns
}
