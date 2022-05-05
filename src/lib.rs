//! TODO
#![feature(iter_intersperse)]
#![warn(missing_docs)]

use std::collections::HashSet;

use sql::Sql;
use verify::{RootTableDefinitions, VerifyError, VerifyTable};

pub mod sql;

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

pub mod fmt;

pub mod verify;

/// This represents a Table in SQL which can actually refer to different underlying thing, like an
/// actual Table in the Database or the result of a SELECT/JOIN Query.
pub trait Table {
    /// Formats the Table using the given Formatter to the final SQL form.
    ///
    /// Supplying different formatters allows for adjusting between different SQL Flavors depending
    /// on the actual Target Database
    fn format<F>(&self, fmt: &mut F) -> Sql
    where
        F: fmt::Formatter;
}
impl Table for &str {
    fn format<F>(&self, _fmt: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        Sql::new(*self)
    }
}
impl VerifyTable for &str {
    fn get_fields(&self, roots: &RootTableDefinitions) -> Result<HashSet<String>, VerifyError> {
        let fields = roots
            .get_table(&self)
            .ok_or_else(|| VerifyError::UnknownTable {
                table: self.to_string(),
            })?;
        Ok(fields.clone())
    }
}
impl Table for String {
    fn format<F>(&self, _: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        Sql::new(self)
    }
}
impl VerifyTable for String {
    fn get_fields(&self, roots: &RootTableDefinitions) -> Result<HashSet<String>, VerifyError> {
        let fields = roots
            .get_table(&self)
            .ok_or_else(|| VerifyError::UnknownTable {
                table: self.to_string(),
            })?;
        Ok(fields.clone())
    }
}

/// Generalises over one or multiple Conditions
pub trait Condition {
    /// Formats the Condition using the provided Formatter
    fn format<F>(&self, fmt: &mut F) -> Option<Sql>
    where
        F: fmt::Formatter;

    /// Combines the current Condition with another condition using a logical and
    fn and<C>(self, other: C) -> And<Self, C>
    where
        Self: Sized,
        C: Condition,
    {
        And::new(self, other)
    }

    /// Combines the current Condition with the other Condition using a logical or
    fn or<C>(self, other: C) -> Or<Self, C>
    where
        Self: Sized,
        C: Condition,
    {
        Or::new(self, other)
    }
}

/// Represents Fields selected by SELECT statement
pub trait Fields {
    /// The Iterator over the Fields
    type FieldIter: Iterator<Item = String>;

    /// Obtains an Iterator over the Fields
    fn to_iterator(&self) -> Self::FieldIter;

    /// Formats the Fields using the provided Formatter
    fn format<F>(&self, fmt: &mut F) -> Sql
    where
        F: fmt::Formatter;
}

/// Represents an SQL Statement, this basically includes all the Things you would use as an End-User,
/// like SELECTs, DELETEs, INSERTs, etc.
pub trait Statement {
    /// Formats the Expression using the provided Formatter
    fn format<F>(&self, fmt: &mut F) -> Sql
    where
        F: fmt::Formatter;
}
