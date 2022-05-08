use crate::{sql::Sql, Order};

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
        base: Sql,
        fields: Sql,
        predicate: Option<Sql>,
        ordering: Option<(Sql, Order)>,
    ) -> crate::sql::Sql {
        let with_where = match predicate {
            Some(pred) => Sql::new(format!("SELECT {} FROM ({}) WHERE {}", fields, base, pred)),
            None => Sql::new(format!("SELECT {} FROM ({})", fields, base)),
        };

        match ordering {
            Some((ord_sql, order)) => {
                Sql::new(format!("{} ORDER BY {} {}", with_where, ord_sql, order))
            }
            None => with_where,
        }
    }

    fn format_parameter(&self) -> Sql {
        Sql::new("?")
    }
}
