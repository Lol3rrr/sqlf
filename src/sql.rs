//! A simple Collection of SQL related Data-Types

use std::fmt::{Debug, Display};

/// A Wrapper encapsulating valid/generated SQL to make sure it stays consistent (even at the type-level)
#[derive(Debug)]
pub struct Sql {
    content: String,
}

impl Sql {
    pub(crate) fn new<C>(content: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            content: content.into(),
        }
    }
}

impl Display for Sql {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.content, f)
    }
}

impl From<Sql> for String {
    fn from(other: Sql) -> Self {
        other.content
    }
}

/// SQL Types that can be used for Columns
pub enum Types {
    /// Stores a String
    String,
    /// Stores Binary Data
    Binary,
}
