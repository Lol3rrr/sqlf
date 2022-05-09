use crate::{Identifier, Predicate, Statement};

pub struct Delete<P> {
    table: Identifier,
    predicate: P,
}
impl<P> Delete<P>
where
    P: Predicate,
{
    pub fn new(table: Identifier, predicate: P) -> Self {
        Self { table, predicate }
    }
}
impl<P> Statement for Delete<P>
where
    P: Predicate,
{
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        todo!()
    }
}
