//! # Raw bindings for SQLite3

#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod constants;
mod functions;
mod types;
mod utility;

pub use constants::*;
pub use functions::*;
pub use types::{SQLite3, SQLite3Int64, SQLite3Stmt, SQLite3UInt64};
pub use utility::SQLiteError;
