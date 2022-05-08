use crate::{fmt::Formatter, sql::Sql, And, Or};

/// A Parameter for a Query
pub struct Parameter {}
impl Expression for Parameter {
    fn format(&self, fmt: &Formatter) -> Sql {
        fmt.parameter()
    }
}

/// An expression
pub trait Expression {
    fn format(&self, fmt: &Formatter) -> Sql;
}

/// Predicates allow for using Conditionals
pub trait Predicate {
    fn format(&self, fmt: &Formatter) -> Option<Sql>;

    fn and<O>(self, other: O) -> And<Self, O>
    where
        Self: Sized,
        O: Predicate,
    {
        And::new(self, other)
    }

    fn or<O>(self, other: O) -> Or<Self, O>
    where
        Self: Sized,
        O: Predicate,
    {
        Or::new(self, other)
    }
}

/// A Query retrieves Data
pub trait Query {
    fn format(&self, fmt: &Formatter) -> Sql;
}

/// A Statement updates or inserts Data
pub trait Statement {}

/// This Trait is used to differentiate between Queries and Table names as both can be used for the
/// Base of the Select Operation
pub trait SelectBase {
    fn format(&self, fmt: &Formatter) -> Sql;
}