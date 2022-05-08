use crate::{Identifier, Predicate, Query, SelectBase};

/// A simple Select Query
///
/// # Example
/// ```rust
/// use sqlf::{Identifier, Select};
///
/// let query = Select::new(Identifier::from("some_table"), vec![Identifier::from("first")], ());
/// ```
pub struct Select<B, P> {
    base: B,
    fields: Vec<Identifier>,
    predicate: P,
}

impl<B, P> Select<B, P>
where
    B: SelectBase,
    P: Predicate,
{
    pub fn new<F>(base: B, fields: F, predicate: P) -> Self
    where
        F: Into<Vec<Identifier>>,
    {
        Self {
            base,
            fields: fields.into(),
            predicate,
        }
    }
}
impl<B, P> Query for Select<B, P>
where
    B: SelectBase,
    P: Predicate,
{
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        fmt.select()
            .base(&self.base)
            .fields(&self.fields)
            .predicate(&self.predicate)
            .finish()
    }
}
impl<B, P> SelectBase for Select<B, P>
where
    B: SelectBase,
    P: Predicate,
{
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        Query::format(self, fmt)
    }
}
