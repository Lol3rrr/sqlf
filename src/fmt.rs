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

    pub fn select(&self) -> FormatSelect<'_> {
        FormatSelect::new(self)
    }
    pub fn insert(&self) -> FormatInsert<'_> {
        FormatInsert::new(self)
    }
    pub fn delete(&self) -> FormatDelete<'_> {
        FormatDelete::new(self)
    }
    pub fn update(&self) -> FormatUpdate<'_> {
        FormatUpdate::new(self)
    }

    pub fn string_literal(&self, value: &str) -> Sql {
        Sql::new(format!("\"{}\"", value))
    }
    pub fn number_literal(&self, numb: i64) -> Sql {
        Sql::new(format!("{}", numb))
    }

    pub fn parameter(&self) -> Sql {
        self.backend.format_parameter()
    }
}

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

    pub fn base<B>(mut self, base: &B) -> Self
    where
        B: SelectBase,
    {
        self.base = Some(base.format(self.formatter));

        self
    }

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

    pub fn predicate<P>(mut self, predicate: &P) -> Self
    where
        P: Predicate,
    {
        self.predicate = predicate.format(self.formatter);
        self
    }

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

    pub fn finish(self) -> Sql {
        let base = self.base.unwrap();
        let fields = self.fields.unwrap();
        let predicate = self.predicate;

        self.backend
            .format_select(base, fields, predicate, self.ordering)
    }
}

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

    pub fn table(mut self, table: &Identifier) -> Self {
        self.table = Some(Sql::new(table.as_ref()));
        self
    }

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

    pub fn finish(self) -> Sql {
        let table = self.table.expect("FormatInsert should have a table set");
        let fields = self
            .field_values
            .expect("FormatInsert should have Field-Values set");

        self.backend.format_insert(table, fields)
    }
}

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

    pub fn table(mut self, table: &Identifier) -> Self {
        self.table = Some(Sql::new(table.as_ref()));
        self
    }

    pub fn predicate<P>(mut self, pred: &P) -> Self
    where
        P: Predicate,
    {
        self.predicate = pred.format(self.formatter);
        self
    }

    pub fn finish(self) -> Sql {
        let table = self
            .table
            .expect("The Table should be set for a DELETE Statement");

        self.backend.format_delete(table, self.predicate)
    }
}

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

    pub fn table(mut self, table: &Identifier) -> Self {
        self.table = Some(Sql::new(table.as_ref()));
        self
    }

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

    pub fn predicate<P>(mut self, predicate: &P) -> Self
    where
        P: Predicate,
    {
        self.predicate = predicate.format(self.formatter);
        self
    }

    pub fn finish(self) -> Sql {
        let table = self.table.expect("");
        let field_values = self.field_values.expect("");

        self.backend
            .format_update(table, field_values, self.predicate)
    }
}
