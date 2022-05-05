use std::collections::HashSet;

use crate::{
    fmt::{FmtBuilder, SelectBuilder},
    sql::Sql,
    verify::{RootTableDefinitions, VerifyError, VerifyTable},
    Condition, Fields, Table,
};

/// A Select Statement
pub struct Select<T, F, C>
where
    T: Table,
    F: Fields,
    C: Condition,
{
    table: T,
    fields: F,
    condition: C,
}

impl<T, F, C> Table for Select<T, F, C>
where
    T: Table,
    F: Fields,
    C: Condition,
{
    fn format<FF>(&self, fmt: &mut FF) -> Sql
    where
        FF: crate::fmt::Formatter,
    {
        fmt.select()
            .table(&self.table)
            .fields(&self.fields)
            .condition(&self.condition)
            .finish()
    }
}
impl<T, F, C> VerifyTable for Select<T, F, C>
where
    T: VerifyTable,
    F: Fields,
    C: Condition,
{
    fn get_fields(
        &self,
        roots: &RootTableDefinitions,
    ) -> Result<std::collections::HashSet<String>, VerifyError> {
        let fields = self.table.get_fields(roots)?;

        let mut result = HashSet::new();
        for field in self.fields.to_iterator() {
            fields
                .get(&field)
                .ok_or_else(|| VerifyError::MissingField {
                    field: field.clone(),
                })?;
            result.insert(field);
        }

        Ok(result)
    }
}

impl<T, F, C> Select<T, F, C>
where
    T: Table,
    F: Fields,
    C: Condition,
{
    /// Creates a new Select Query
    pub fn new(table: T, condition: C, fields: F) -> Self {
        Self {
            table,
            fields,
            condition,
        }
    }
}
