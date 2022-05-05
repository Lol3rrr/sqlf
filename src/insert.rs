use crate::Table;

/// An Insert Statement
pub struct Insert<T>
where
    T: Table,
{
    table: T,
    values: Vec<(String, String)>,
}
