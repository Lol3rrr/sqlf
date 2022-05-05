use crate::{Condition, Table};

/// A Delete Statement
pub struct Delete<T, C>
where
    T: Table,
    C: Condition,
{
    table: T,
    condition: C,
}
