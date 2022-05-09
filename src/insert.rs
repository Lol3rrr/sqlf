use crate::{Expression, Identifier, Statement};

pub struct Insert {
    table: Identifier,
    values: Vec<(Identifier, Box<dyn Expression>)>,
}
impl Insert {
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
