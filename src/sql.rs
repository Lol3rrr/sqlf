pub struct Sql {
    content: String,
}

impl Sql {
    pub(crate) fn new<C>(content: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            content: content.into(),
        }
    }
}
