use std::collections::HashSet;

use crate::{sql::Types, Statement};

/// Creates a new Table with the given Configuration
pub struct CreateTable {
    name: String,
    /// A List of (Name, Type) Tuples representing the Columns
    columns: Vec<(String, Types)>,
}

impl CreateTable {
    /// Creates a new CreateTable Expression
    pub fn new<N, C>(name: N, columns: C) -> Self
    where
        N: Into<String>,
        C: Into<Vec<(String, Types)>>,
    {
        Self {
            name: name.into(),
            columns: columns.into(),
        }
    }

    /// Gets a Set of the Columns of the Table
    pub fn columns(&self) -> HashSet<String> {
        self.columns.iter().map(|(n, _)| n.to_string()).collect()
    }
}

impl Statement for CreateTable {
    fn format<F>(&self, _: &mut F) -> crate::sql::Sql
    where
        F: crate::fmt::Formatter,
    {
        dbg!(&self.name);
        todo!()
    }
}
