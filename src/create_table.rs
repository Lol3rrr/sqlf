use std::collections::HashSet;

/// Creates a new Table with the given Configuration
pub struct CreateTable {
    /// A List of (Name, Type) Tuples representing the Columns
    columns: Vec<(String, String)>,
}

impl CreateTable {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    /// Gets a Set of the Columns of the Table
    pub fn columns(&self) -> HashSet<String> {
        self.columns.iter().map(|(n, _)| n.to_string()).collect()
    }
}
