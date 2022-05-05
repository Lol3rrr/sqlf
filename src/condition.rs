use crate::{
    fmt::{self, ConditionBuilder},
    Condition,
};

impl Condition for () {
    fn format<F>(&self, _: &mut F) -> Option<String>
    where
        F: fmt::Formatter,
    {
        None
    }
}

impl<S1, S2> Condition for (S1, S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    fn format<F>(&self, fmt: &mut F) -> Option<String>
    where
        F: fmt::Formatter,
    {
        Some(
            fmt.condition()
                .equal(self.0.as_ref().to_string(), self.1.as_ref().to_string()),
        )
    }
}

/// Combines the Two Conditions using AND
pub struct And<L, R>
where
    L: Condition,
    R: Condition,
{
    left: L,
    right: R,
}

impl<L, R> And<L, R>
where
    L: Condition,
    R: Condition,
{
    /// Creates a new And Condition with the two given Conditions
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Condition for And<L, R>
where
    L: Condition,
    R: Condition,
{
    fn format<F>(&self, fmt: &mut F) -> Option<String>
    where
        F: fmt::Formatter,
    {
        let left_str = self.left.format(fmt)?;
        let right_str = self.right.format(fmt)?;

        Some(fmt.condition().and(left_str, right_str))
    }
}

/// Combines two conditionals using logical or
pub struct Or<L, R>
where
    L: Condition,
    R: Condition,
{
    left: L,
    right: R,
}

impl<L, R> Or<L, R>
where
    L: Condition,
    R: Condition,
{
    /// Creates a new Or
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Condition for Or<L, R>
where
    L: Condition,
    R: Condition,
{
    fn format<F>(&self, fmt: &mut F) -> Option<String>
    where
        F: fmt::Formatter,
    {
        let left_str = self.left.format(fmt)?;
        let right_str = self.right.format(fmt)?;

        Some(fmt.condition().or(left_str, right_str))
    }
}
