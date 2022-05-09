use crate::{
    sql::Sql, Expression, Identifier, Order, OrderExpression, Predicate, Query, SelectBase,
    Statement,
};

mod sqlite;
pub use sqlite::SqliteBackend;

pub trait FormatBackend {
    fn format_select(
        &self,
        base: Sql,
        fields: Sql,
        predicate: Option<Sql>,
        ordering: Option<(Sql, Order)>,
    ) -> Sql;

    fn format_insert(&self, table: Sql, fields: Vec<(Sql, Sql)>) -> Sql;

    fn format_parameter(&self) -> Sql;
}

pub struct Formatter {
    backend: Box<dyn FormatBackend>,
}

impl Formatter {
    pub fn new<FB>(backend: FB) -> Self
    where
        FB: FormatBackend + 'static,
    {
        Self {
            backend: Box::new(backend),
        }
    }

    pub fn formatq<Q>(&self, query: &Q) -> Sql
    where
        Q: Query,
    {
        query.format(self)
    }
    pub fn formats<S>(&self, stmnt: &S) -> Sql
    where
        S: Statement,
    {
        stmnt.format(self)
    }

    pub fn select(&self) -> FormatSelect<'_> {
        FormatSelect::new(self)
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
    pub fn insert(&self) -> FormatInsert<'_> {
        FormatInsert::new(self)
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
