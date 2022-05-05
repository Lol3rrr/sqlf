use crate::{
    fmt::{self, FieldsBuilder, FmtBuilder},
    sql::Sql,
    Fields,
};

impl Fields for &str {
    type FieldIter = std::iter::Once<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        std::iter::once(self.to_string())
    }

    fn format<F>(&self, _: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        todo!()
    }
}
impl Fields for String {
    type FieldIter = std::iter::Once<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        std::iter::once(self.clone())
    }

    fn format<F>(&self, _: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        todo!()
    }
}

impl Fields for Vec<String> {
    type FieldIter = std::vec::IntoIter<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        self.clone().into_iter()
    }

    fn format<F>(&self, fmt: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        let mut fields = fmt.fields();

        for entry in self.iter() {
            fields.add_field(entry.to_string());
        }

        fields.finish()
    }
}

impl Fields for &[&str] {
    type FieldIter = std::vec::IntoIter<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        self.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn format<F>(&self, _: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        todo!()
    }
}
impl Fields for &[String] {
    type FieldIter = std::vec::IntoIter<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        self.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn format<F>(&self, _: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        todo!()
    }
}

impl<const N: usize> Fields for &[&str; N] {
    type FieldIter = std::vec::IntoIter<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        self.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn format<F>(&self, fmt: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        let mut fields = fmt.fields();

        for field in self.iter() {
            fields.add_field(field.to_string());
        }

        fields.finish()
    }
}

/// Represents all Fields available on the underlying Table
pub struct AllFields {}
impl Fields for AllFields {
    type FieldIter = std::iter::Empty<String>;

    fn to_iterator(&self) -> Self::FieldIter {
        std::iter::empty()
    }

    fn format<F>(&self, _: &mut F) -> Sql
    where
        F: fmt::Formatter,
    {
        todo!()
    }
}
