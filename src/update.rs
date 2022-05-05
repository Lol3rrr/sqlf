use crate::{Condition, Table};

/// A single Update Statement
pub struct Update<T, C>
where
    T: Table,
    C: Condition,
{
    table: T,
    condition: C,
    values: Vec<(String, String)>,
}
