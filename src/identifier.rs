use crate::{sql::Sql, OrderExpression, SelectBase};

/// An Identifier, like a Table or Column Name
#[derive(Debug, Clone)]
pub struct Identifier {
    name: String,
}
impl Identifier {
    /// Creates a new Identifier
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self { name: name.into() }
    }
}
impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Self { name }
    }
}
impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl From<Identifier> for String {
    fn from(raw: Identifier) -> Self {
        raw.name
    }
}
impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
impl SelectBase for Identifier {
    fn format(&self, _: &crate::fmt::Formatter) -> Sql {
        Sql::new(self.name.clone())
    }
}
impl OrderExpression for Identifier {
    fn format(&self, _: &crate::fmt::Formatter) -> Option<Sql> {
        Some(Sql::new(&self.name))
    }
}
