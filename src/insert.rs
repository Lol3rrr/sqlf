use crate::{Expression, Identifier, Statement};

/// The Insert Statement
pub struct Insert {
    table: Identifier,
    values: Vec<(Identifier, Box<dyn Expression>)>,
}
impl Insert {
    /// Creates a new Statement
    pub fn new(table: Identifier, values: Vec<(Identifier, Box<dyn Expression>)>) -> Self {
        Self { table, values }
    }
}
impl Statement for Insert {
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        fmt.insert()
            .table(&self.table)
            .field_values(
                self.values
                    .iter()
                    .map(|(id, exp)| (id.clone(), exp.boxed())),
            )
            .finish()
    }
}
