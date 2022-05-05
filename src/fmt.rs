//! TODO

use crate::{Condition, Fields, Table};

pub mod sqlite;

/// An underlying Trait for the Formatting
pub trait FmtBuilder {
    /// Converts the current Builder into the final SQL String
    fn finish(&mut self) -> String;
}

/// The Trait describing the Target Formatters
pub trait Formatter {
    /// The Builder for Select Queries
    type SelectBuilder: SelectBuilder;
    /// The Builder for Fields
    type FieldsBuilder: FieldsBuilder;
    /// The Builder for Conditionals in a Query
    type ConditionBuilder: ConditionBuilder;

    /// Obtains a Select Builder
    fn select(&mut self) -> Self::SelectBuilder;
    /// Obtains a Fields Builder
    fn fields(&mut self) -> Self::FieldsBuilder;
    /// Obtains a Conditional Builder
    fn condition(&mut self) -> Self::ConditionBuilder;
}

/// The Builder Trait for Select Queries
pub trait SelectBuilder: FmtBuilder {
    /// The Table used for the Select Query
    fn table<'s, 'o, T>(&'s mut self, table: &T) -> &'o mut Self
    where
        's: 'o,
        T: Table;

    /// The Fields being selected
    fn fields<'s, 'o, F>(&'s mut self, fields: &F) -> &'o mut Self
    where
        's: 'o,
        F: Fields;

    /// The Condition for the Select
    fn condition<'s, 'o, C>(&'s mut self, condition: &C) -> &'o mut Self
    where
        's: 'o,
        C: Condition;
}

/// The Builder Trait for Fields
pub trait FieldsBuilder: FmtBuilder {
    /// Adds a new Field
    fn add_field(&mut self, name: String);
}

/// The Builder Trait for Conditionals
pub trait ConditionBuilder: FmtBuilder {
    /// Generates the SQL for an Equals comparison between the Two elements
    fn equal(self, left: String, right: String) -> String;

    /// Combines the two sides with a logical AND
    fn and(self, left: String, right: String) -> String;

    /// Combines the two sides with a logical OR
    fn or(self, left: String, right: String) -> String;
}
