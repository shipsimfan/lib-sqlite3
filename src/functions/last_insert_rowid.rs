use crate::{SQLite3, SQLite3Int64};

#[cfg_attr(target_os = "windows", link(name = "sqlite3-win"))]

extern "C" {
    pub fn sqlite3_last_insert_rowid(db: *mut SQLite3) -> SQLite3Int64;
}
