//! TODO

use crate::{Condition, Fields, Select, Table};

use super::{ConditionBuilder, FieldsBuilder, FmtBuilder, Formatter, SelectBuilder};

/// An SQL-Formatter that targets SQLite
pub struct SqliteFormatter {}

impl SqliteFormatter {
    /// Creates a new Instance of the Formatter
    pub fn new() -> Self {
        Self {}
    }

    /// Formats the given Select Statement
    pub fn format_select<T, F, C>(&mut self, s: &Select<T, F, C>) -> String
    where
        T: Table,
        F: Fields,
        C: Condition,
    {
        s.format(self)
    }

    fn duplicate(&self) -> Self {
        Self {}
    }
}

impl Default for SqliteFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for SqliteFormatter {
    type SelectBuilder = SqliteSelectBuilder;
    type FieldsBuilder = SqliteFieldsBuilder;
    type ConditionBuilder = SqliteConditionBuilder;

    fn select(&mut self) -> Self::SelectBuilder {
        SqliteSelectBuilder::new(self)
    }
    fn fields(&mut self) -> Self::FieldsBuilder {
        SqliteFieldsBuilder::new()
    }
    fn condition(&mut self) -> Self::ConditionBuilder {
        SqliteConditionBuilder::new()
    }
}

/// The Builder for Select Statements
pub struct SqliteSelectBuilder {
    table: Option<String>,
    fields: Option<String>,
    condition: Option<String>,
    root: SqliteFormatter,
}

impl SqliteSelectBuilder {
    fn new(root: &SqliteFormatter) -> Self {
        Self {
            table: None,
            fields: None,
            condition: None,
            root: root.duplicate(),
        }
    }
}

impl FmtBuilder for SqliteSelectBuilder {
    fn finish(&mut self) -> String {
        let table_str = self.table.as_ref().expect("The Table should be set");
        let field_str = self.fields.as_ref().expect("The Fields should be set");

        match self.condition.as_ref() {
            Some(cond) if !cond.is_empty() => {
                format!("SELECT {} FROM ({}) WHERE {}", field_str, table_str, cond)
            }
            _ => {
                format!("SELECT {} FROM ({})", field_str, table_str)
            }
        }
    }
}
impl SelectBuilder for SqliteSelectBuilder {
    fn table<'s, 'o, T>(&'s mut self, table: &T) -> &'o mut Self
    where
        's: 'o,
        T: Table,
    {
        self.table = Some(table.format(&mut self.root));
        self
    }
    fn fields<'s, 'o, F>(&'s mut self, fields: &F) -> &'o mut Self
    where
        's: 'o,
        F: Fields,
    {
        self.fields = Some(fields.format(&mut self.root));
        self
    }
    fn condition<'s, 'o, C>(&'s mut self, condition: &C) -> &'o mut Self
    where
        's: 'o,
        C: Condition,
    {
        self.condition = condition.format(&mut self.root);
        self
    }
}

/// The Builder for Fields
pub struct SqliteFieldsBuilder {
    entries: Vec<String>,
}

impl SqliteFieldsBuilder {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl FmtBuilder for SqliteFieldsBuilder {
    fn finish(&mut self) -> String {
        let raw_inner = self
            .entries
            .iter()
            .map(|s| s.as_str())
            .intersperse(",")
            .collect::<String>();

        format!("({})", raw_inner)
    }
}
impl FieldsBuilder for SqliteFieldsBuilder {
    fn add_field(&mut self, name: String) {
        self.entries.push(name);
    }
}

/// The Builder for a Condition
pub struct SqliteConditionBuilder {}

impl SqliteConditionBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl FmtBuilder for SqliteConditionBuilder {
    fn finish(&mut self) -> String {
        todo!()
    }
}
impl ConditionBuilder for SqliteConditionBuilder {
    fn equal(self, left: String, right: String) -> String {
        format!("{}={}", left, right)
    }

    fn and(self, left: String, right: String) -> String {
        format!("({}) AND ({})", left, right)
    }

    fn or(self, left: String, right: String) -> String {
        format!("({}) OR ({})", left, right)
    }
}
