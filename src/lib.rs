//! # Raw bindings for SQLite3

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod constants;
mod functions;
mod types;
mod utility;

pub use constants::*;
pub use functions::{
    sqlite3_close, sqlite3_errstr, sqlite3_open, sqlite3_prepare_v2, sqlite3_step,
};
pub use types::{SQLite3, SQLite3Stmt};
pub use utility::SQLiteError;
