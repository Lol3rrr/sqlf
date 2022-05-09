//! Contains all the Formatting related things.
//! This mainly concerns how we can provide a uniform interface to create the final Queries
//! regardless of the flavor of the underlying target database.

use crate::{
    sql::Sql, Expression, Identifier, Order, OrderExpression, Predicate, Query, SelectBase,
    Statement,
};

mod sqlite;
pub use sqlite::SqliteBackend;

/// Describes the Interface for a Formatting Backend
pub trait FormatBackend {
    /// Formats a Select Query with the given information
    fn format_select(
        &self,
        base: Sql,
        fields: Sql,
        predicate: Option<Sql>,
        ordering: Option<(Sql, Order)>,
    ) -> Sql;

    /// Formats an Insert Statement with the given Information
    fn format_insert(&self, table: Sql, fields: Vec<(Sql, Sql)>) -> Sql;

    /// Formats a Delete Statement with the given Information
    fn format_delete(&self, table: Sql, predicate: Option<Sql>) -> Sql;

    /// Formats an Update Statement with the given Information
    fn format_update(&self, table: Sql, fields: Vec<(Sql, Sql)>, predicate: Option<Sql>) -> Sql;

    /// Formats an anonymus Parameter
    fn format_parameter(&self) -> Sql;
}

/// The Formatter
pub struct Formatter {
    backend: Box<dyn FormatBackend>,
}

impl Formatter {
    /// Creates a new Formatter using the given Backend for final formatting
    pub fn new<FB>(backend: FB) -> Self
    where
        FB: FormatBackend + 'static,
    {
        Self {
            backend: Box::new(backend),
        }
    }

    /// Formats the given Query
    pub fn formatq<Q>(&self, query: &Q) -> Sql
    where
        Q: Query,
    {
        query.format(self)
    }

    /// Formats the given Statement
    pub fn formats<S>(&self, stmnt: &S) -> Sql
    where
        S: Statement,
    {
        stmnt.format(self)
    }

    /// Obtains a Select-Formatter
    pub fn select(&self) -> FormatSelect<'_> {
        FormatSelect::new(self)
    }
    /// Obtains an Insert-Formatter
    pub fn insert(&self) -> FormatInsert<'_> {
        FormatInsert::new(self)
    }
    /// Obtains a Delete-Formatter
    pub fn delete(&self) -> FormatDelete<'_> {
        FormatDelete::new(self)
    }
    /// Obtains an Update-Formatter
    pub fn update(&self) -> FormatUpdate<'_> {
        FormatUpdate::new(self)
    }

    /// Formats a String-Literal
    pub fn string_literal(&self, value: &str) -> Sql {
        Sql::new(format!("\"{}\"", value))
    }
    /// Formats a Number-Literal
    pub fn number_literal(&self, numb: i64) -> Sql {
        Sql::new(format!("{}", numb))
    }

    /// Obtains the Sql for an anonymous Parameter
    pub fn parameter(&self) -> Sql {
        self.backend.format_parameter()
    }
}

/// The Formatter to configure a Select Query
pub struct FormatSelect<'b> {
    formatter: &'b Formatter,
    backend: &'b dyn FormatBackend,

    base: Option<Sql>,
    fields: Option<Sql>,
    predicate: Option<Sql>,
    ordering: Option<(Sql, Order)>,
}

impl<'b> FormatSelect<'b> {
    fn new(formatter: &'b Formatter) -> Self {
        Self {
            formatter,
            backend: formatter.backend.as_ref(),
            base: None,
            fields: None,
            predicate: None,
            ordering: None,
        }
    }

    /// Updates the Base of the Select Query
    pub fn base<B>(mut self, base: &B) -> Self
    where
        B: SelectBase,
    {
        self.base = Some(base.format(self.formatter));

        self
    }

    /// Updates the Fields that should be selected
    pub fn fields(mut self, fields: &[Identifier]) -> Self {
        let field_str = {
            let mut result = String::new();
            let mut tmp = fields
                .iter()
                .map(|id| SelectBase::format(id, self.formatter))
                .peekable();

            let mut next = tmp.next();
            while next.is_some() && tmp.peek().is_some() {
                result.push_str(&format!("{}", next.unwrap()));
                result.push(',');

                next = tmp.next();
            }

            if let Some(last) = next {
                result.push_str(&format!("{}", last))
            }

            result
        };
        self.fields = Some(Sql::new(field_str));

        self
    }

    /// Updates the Predicate for the Query
    pub fn predicate<P>(mut self, predicate: &P) -> Self
    where
        P: Predicate,
    {
        self.predicate = predicate.format(self.formatter);
        self
    }

    /// Updates the Ordering of the Results
    pub fn order_by<O>(mut self, ordering: &(O, Order)) -> Self
    where
        O: OrderExpression,
    {
        match ordering.0.format(self.formatter) {
            Some(ord) => {
                self.ordering = Some((ord, ordering.1.clone()));
                self
            }
            None => self,
        }
    }

    /// Converts the configured Formatter into the final SQL
    pub fn finish(self) -> Sql {
        let base = self.base.unwrap();
        let fields = self.fields.unwrap();
        let predicate = self.predicate;

        self.backend
            .format_select(base, fields, predicate, self.ordering)
    }
}

/// The Formatter to configure an Insert Statement
pub struct FormatInsert<'b> {
    formatter: &'b Formatter,
    backend: &'b dyn FormatBackend,

    table: Option<Sql>,
    field_values: Option<Vec<(Sql, Sql)>>,
}

impl<'b> FormatInsert<'b> {
    fn new(formatter: &'b Formatter) -> Self {
        Self {
            formatter,
            backend: formatter.backend.as_ref(),

            table: None,
            field_values: None,
        }
    }

    /// Updates the Table on which the Statement should be executed on
    pub fn table(mut self, table: &Identifier) -> Self {
        self.table = Some(Sql::new(table.as_ref()));
        self
    }

    /// Updates the Fields and the Values that should be inserted
    pub fn field_values<I>(mut self, field_val_iter: I) -> Self
    where
        I: Iterator<Item = (Identifier, Box<dyn Expression>)>,
    {
        let fields: Vec<_> = field_val_iter
            .map(|(id, exp)| (Sql::new(id.as_ref()), exp.format(self.formatter)))
            .collect();
        self.field_values = Some(fields);

        self
    }

    /// Converts the configured Formatter into the final SQL
    pub fn finish(self) -> Sql {
        let table = self.table.expect("FormatInsert should have a table set");
        let fields = self
            .field_values
            .expect("FormatInsert should have Field-Values set");

        self.backend.format_insert(table, fields)
    }
}

/// The Formatter to configure a Delete Statement
pub struct FormatDelete<'b> {
    formatter: &'b Formatter,
    backend: &'b dyn FormatBackend,

    table: Option<Sql>,
    predicate: Option<Sql>,
}

impl<'b> FormatDelete<'b> {
    fn new(formatter: &'b Formatter) -> Self {
        Self {
            formatter,
            backend: formatter.backend.as_ref(),

            table: None,
            predicate: None,
        }
    }

    /// Updates the Table from which entries should be deleted
    pub fn table(mut self, table: &Identifier) -> Self {
        self.table = Some(Sql::new(table.as_ref()));
        self
    }

    /// Updates the Predicate that determines which Rows should be deleted
    pub fn predicate<P>(mut self, pred: &P) -> Self
    where
        P: Predicate,
    {
        self.predicate = pred.format(self.formatter);
        self
    }

    /// Converts the configured Formatter into the final SQL
    pub fn finish(self) -> Sql {
        let table = self
            .table
            .expect("The Table should be set for a DELETE Statement");

        self.backend.format_delete(table, self.predicate)
    }
}

/// The Formatter to configure an Update Statement
pub struct FormatUpdate<'b> {
    formatter: &'b Formatter,
    backend: &'b dyn FormatBackend,

    table: Option<Sql>,
    field_values: Option<Vec<(Sql, Sql)>>,
    predicate: Option<Sql>,
}

impl<'b> FormatUpdate<'b> {
    fn new(formatter: &'b Formatter) -> Self {
        Self {
            formatter,
            backend: formatter.backend.as_ref(),

            table: None,
            field_values: None,
            predicate: None,
        }
    }

    /// Updates the Table on which this Statement should be executed
    pub fn table(mut self, table: &Identifier) -> Self {
        self.table = Some(Sql::new(table.as_ref()));
        self
    }

    /// Updates the Fields and the Values with which the Update should be executed
    pub fn field_values<I>(mut self, field_values: I) -> Self
    where
        I: Iterator<Item = (Identifier, Box<dyn Expression>)>,
    {
        self.field_values = Some(
            field_values
                .map(|(id, exp)| (Sql::new(id.as_ref()), exp.format(self.formatter)))
                .collect(),
        );
        self
    }

    /// Updates the Predicate used for determining which entries should be updated
    pub fn predicate<P>(mut self, predicate: &P) -> Self
    where
        P: Predicate,
    {
        self.predicate = predicate.format(self.formatter);
        self
    }

    /// Converts the configured Formatter into the final SQL
    pub fn finish(self) -> Sql {
        let table = self.table.expect("");
        let field_values = self.field_values.expect("");

        self.backend
            .format_update(table, field_values, self.predicate)
    }
}
