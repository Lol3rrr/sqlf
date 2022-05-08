use crate::{fmt::Formatter, sql::Sql, Expression, Identifier, Literal, Predicate, SelectBase};

impl Predicate for () {
    fn format(&self, _: &Formatter) -> Option<Sql> {
        None
    }
}

impl Predicate for (Identifier, Literal) {
    fn format(&self, fmt: &Formatter) -> Option<Sql> {
        let ident = self.0.format(fmt);
        let exp = self.1.format(fmt);
        Some(Sql::new(format!("{}={}", ident, exp)))
    }
}

pub struct And<L, R> {
    left: L,
    right: R,
}

impl<L, R> And<L, R>
where
    L: Predicate,
    R: Predicate,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Predicate for And<L, R>
where
    L: Predicate,
    R: Predicate,
{
    fn format(&self, fmt: &Formatter) -> Option<Sql> {
        let left_pred = self.left.format(fmt);
        let right_pred = self.right.format(fmt);

        match (left_pred, right_pred) {
            (Some(l_p), Some(r_p)) => Some(Sql::new(format!("{} AND {}", l_p, r_p))),
            (Some(l_p), None) => Some(l_p),
            (None, Some(r_p)) => Some(r_p),
            (None, None) => None,
        }
    }
}

pub struct Or<L, R> {
    left: L,
    right: R,
}

impl<L, R> Or<L, R>
where
    L: Predicate,
    R: Predicate,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Predicate for Or<L, R>
where
    L: Predicate,
    R: Predicate,
{
    fn format(&self, fmt: &Formatter) -> Option<Sql> {
        let left_pred = self.left.format(fmt);
        let right_pred = self.right.format(fmt);

        match (left_pred, right_pred) {
            (Some(l_p), Some(r_p)) => Some(Sql::new(format!("{} OR {}", l_p, r_p))),
            (Some(l_p), None) => Some(l_p),
            (None, Some(r_p)) => Some(r_p),
            (None, None) => None,
        }
    }
}
