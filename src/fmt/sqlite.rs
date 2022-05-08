use crate::sql::Sql;

use super::FormatBackend;

pub struct SqliteBackend {}

impl SqliteBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SqliteBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatBackend for SqliteBackend {
    fn format_select(
        &self,
        base: crate::sql::Sql,
        fields: crate::sql::Sql,
        predicate: Option<crate::sql::Sql>,
    ) -> crate::sql::Sql {
        match predicate {
            Some(pred) => Sql::new(format!("SELECT {} FROM ({}) WHERE {}", fields, base, pred)),
            None => Sql::new(format!("SELECT {} FROM ({})", fields, base)),
        }
    }

    fn format_parameter(&self) -> Sql {
        Sql::new("?")
    }
}
