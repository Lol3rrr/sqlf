use crate::{sql::Sql, Identifier, Order, OrderExpression, Predicate, Query, SelectBase};

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

    pub fn select(&self) -> FormatSelect<'_> {
        FormatSelect::new(self)
    }

    pub fn string_literal(&self, value: &str) -> Sql {
        Sql::new(format!("\"{}\"", value))
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
