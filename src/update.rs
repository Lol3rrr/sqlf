use crate::{Expression, Identifier, Predicate, Statement};

/// The Update Statement
pub struct Update<P> {
    table: Identifier,
    values: Vec<(Identifier, Box<dyn Expression>)>,
    predicate: P,
}
impl<P> Update<P>
where
    P: Predicate,
{
    /// Creates a new Query based on the given Information
    pub fn new(
        table: Identifier,
        values: Vec<(Identifier, Box<dyn Expression>)>,
        predicate: P,
    ) -> Self {
        Self {
            table,
            predicate,
            values,
        }
    }
}
impl<P> Statement for Update<P>
where
    P: Predicate,
{
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        fmt.update()
            .table(&self.table)
            .field_values(
                self.values
                    .iter()
                    .map(|(id, exp)| (id.clone(), exp.boxed())),
            )
            .predicate(&self.predicate)
            .finish()
    }
}
