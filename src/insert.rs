use crate::{Expression, Identifier, Predicate, Statement};

pub struct Insert<P> {
    table: Identifier,
    values: Vec<(Identifier, Box<dyn Expression>)>,
    predicate: P,
}
impl<P> Insert<P>
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
            values,
            predicate,
        }
    }
}
impl<P> Statement for Insert<P> where P: Predicate {}
