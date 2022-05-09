use crate::{fmt::Formatter, sql::Sql, And, Or};

/// A Parameter for a Query
pub struct Parameter {}
impl Expression for Parameter {
    fn format(&self, fmt: &Formatter) -> Sql {
        fmt.parameter()
    }

    fn boxed(&self) -> Box<dyn Expression> {
        Box::new(Self {})
    }
}

/// An expression
pub trait Expression {
    /// Formats the underlying Expression using the Formatter
    fn format(&self, fmt: &Formatter) -> Sql;

    /// Should clone and box the Expression, this signature is needed to make the Trait Object-safe
    fn boxed(&self) -> Box<dyn Expression>;
}

/// An Expression for Ordering
pub trait OrderExpression {
    /// Formats the underlying Expression using the Formatter
    fn format(&self, fmt: &Formatter) -> Option<Sql>;
}
impl OrderExpression for () {
    fn format(&self, _: &Formatter) -> Option<Sql> {
        None
    }
}

/// Predicates allow for using Conditionals
pub trait Predicate {
    /// Formats the Predicate
    fn format(&self, fmt: &Formatter) -> Option<Sql>;

    /// Combines the current Predicate with the other Predicate using a logical AND
    fn and<O>(self, other: O) -> And<Self, O>
    where
        Self: Sized,
        O: Predicate,
    {
        And::new(self, other)
    }

    /// Combines the current Predicate with the other Predicate using a logical OR
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
    /// Formats the entire Query
    fn format(&self, fmt: &Formatter) -> Sql;
}

/// A Statement updates or inserts Data
pub trait Statement {
    /// Formats the entire Statement
    fn format(&self, fmt: &Formatter) -> Sql;
}

/// This Trait is used to differentiate between Queries and Table names as both can be used for the
/// Base of the Select Operation
pub trait SelectBase {
    /// Formats the SelectBase
    fn format(&self, fmt: &Formatter) -> Sql;
}
