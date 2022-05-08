//! A Library designed to make building SQL Queries easy, portable, fast and safe.
//!
//! # Goal
//! The Goal is to be able to write SQL Queries without having to worry about silly typos,
//! differences between SQL for Postgres, SQLite, etc. and being able to use Rust's extensive
//! type system to your advantage
//!
//! # Example
//! ```rust
//! use sqlf::{Select, Identifier, fmt::{Formatter, SqliteBackend}};
//!
//! let formatter = Formatter::new(SqliteBackend::new());
//! let query = Select::new(Identifier::new("users"), vec!["name".into()], ());
//! let sql = formatter.formatq(&query);
//! let sql_str: String = sql.into();
//!
//! assert_eq!("SELECT name FROM (users)".to_string(), sql_str);
//! ```
#![feature(iter_intersperse)]
// #![warn(missing_docs)]

mod traits;
pub use traits::*;

pub mod sql;

mod identifier;
pub use identifier::Identifier;
mod literal;
pub use literal::Literal;

mod predicates;
pub use predicates::{And, Or};

mod select;
pub use select::Select;
mod insert;
pub use insert::Insert;
mod update;
pub use update::Update;
mod delete;
pub use delete::Delete;

/*
mod condition;
pub use condition::{And, Or};
mod fields;
pub use fields::AllFields;

mod delete;
pub use delete::Delete;
mod insert;
pub use insert::Insert;
mod select;
pub use select::Select;
mod update;
pub use update::Update;
mod create_table;
pub use create_table::CreateTable;
*/

pub mod fmt;

// pub mod verify;
