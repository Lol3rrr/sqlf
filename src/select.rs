use crate::{Identifier, Order, OrderExpression, Predicate, Query, SelectBase};

/// A simple Select Query
///
/// # Example
/// ```rust
/// use sqlf::{Identifier, Select};
///
/// let query = Select::new(Identifier::from("some_table"), vec![Identifier::from("first")], ());
/// ```
pub struct Select<B, P, O = ()> {
    base: B,
    fields: Vec<Identifier>,
    predicate: P,
    ordering: (O, Order),
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
            ordering: ((), Order::Ascending),
        }
    }

    pub fn order<O2>(self, ordering: O2, order: Order) -> Select<B, P, O2>
    where
        O2: OrderExpression,
    {
        Select {
            base: self.base,
            fields: self.fields,
            predicate: self.predicate,
            ordering: (ordering, order),
        }
    }
}
impl<B, P, O> Query for Select<B, P, O>
where
    B: SelectBase,
    P: Predicate,
    O: OrderExpression,
{
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        fmt.select()
            .base(&self.base)
            .fields(&self.fields)
            .predicate(&self.predicate)
            .order_by(&self.ordering)
            .finish()
    }
}
impl<B, P, O> SelectBase for Select<B, P, O>
where
    B: SelectBase,
    P: Predicate,
    O: OrderExpression,
{
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        Query::format(self, fmt)
    }
}
