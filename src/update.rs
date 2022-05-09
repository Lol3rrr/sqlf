use crate::{Expression, Identifier, Predicate, Statement};

pub struct Update<P> {
    table: Identifier,
    values: Vec<(Identifier, Box<dyn Expression>)>,
    predicate: P,
}
impl<P> Update<P>
where
    P: Predicate,
{
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
        todo!()
    }
}
