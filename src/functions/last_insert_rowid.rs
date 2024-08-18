use crate::{SQLite3, SQLite3Int64};

#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_last_insert_rowid(db: *mut SQLite3) -> SQLite3Int64;
}
