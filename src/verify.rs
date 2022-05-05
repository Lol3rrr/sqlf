//! TODO

use std::collections::{HashMap, HashSet};

use crate::{Condition, Fields, Select, Table};

/// Defines the Root-Tables in the Database
pub struct RootTableDefinitions {
    inner: HashMap<String, HashSet<String>>,
}

impl RootTableDefinitions {
    /// Creates a new empty Definition of Root-Tables
    pub fn new() -> Self {
        RootTableDefinitions {
            inner: HashMap::new(),
        }
    }

    /// Adds a new Table Definition
    pub fn add_table<N>(&mut self, name: N, columns: HashSet<String>)
    where
        N: Into<String>,
    {
        self.inner.insert(name.into(), columns);
    }

    /// Attempts to load the Definition of a Table with the given Name
    pub fn get_table<N>(&self, name: N) -> Option<&HashSet<String>>
    where
        N: AsRef<str>,
    {
        self.inner.get(name.as_ref())
    }
}

impl Default for RootTableDefinitions {
    fn default() -> Self {
        Self::new()
    }
}

/// An extension Trait for adding the Functionality needed for verifying a given SQL Query
pub trait VerifyTable: Table {
    /// Returns the Fields available in this Table
    fn get_fields(&self, roots: &RootTableDefinitions) -> Result<HashSet<String>, VerifyError>;
}

/// The Error that could be returned when attempting to verify a Query
#[derive(Debug, PartialEq)]
pub enum VerifyError {
    /// A Field was used but not defined on the Table it was used on
    MissingField {
        /// The Field that was not found
        field: String,
    },
    /// A Table was attempted to be used that was not defined
    UnknownTable {
        /// The Name of the Table
        table: String,
    },
}

/// Attempts to verify the given Select Statement
pub fn verify_select<T, F, C>(
    select: Select<T, F, C>,
    base_definition: &RootTableDefinitions,
) -> Result<(), VerifyError>
where
    T: VerifyTable,
    F: Fields,
    C: Condition,
{
    select.get_fields(base_definition).map(|_| ())
}
