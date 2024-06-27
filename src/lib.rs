//! # Raw bindings for SQLite3

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::sqlite3_open;
pub use types::SQLite3;
