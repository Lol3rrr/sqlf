use crate::Expression;

/// A Literal Value
#[derive(Debug, Clone)]
pub enum Literal {
    /// A String Literal
    String(String),
    /// A Number Literal
    Number(i64),
}
impl Expression for Literal {
    fn format(&self, fmt: &crate::fmt::Formatter) -> crate::sql::Sql {
        match self {
            Self::String(content) => fmt.string_literal(content),
            Self::Number(value) => fmt.number_literal(*value),
        }
    }

    fn boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
impl From<String> for Literal {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}
impl From<&str> for Literal {
    fn from(val: &str) -> Self {
        Self::String(val.to_string())
    }
}
impl From<i64> for Literal {
    fn from(val: i64) -> Self {
        Self::Number(val)
    }
}
