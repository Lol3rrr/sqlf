use sqlf::{
    fmt::{Formatter, SqliteBackend},
    Identifier, Insert, Literal,
};

fn main() {
    let formatter = Formatter::new(SqliteBackend::new());

    let insert = Insert::new(
        Identifier::new("test"),
        vec![("field1".into(), Box::new(Literal::from("value")))],
    );
    dbg!(formatter.formats(&insert));

    let insert2 = Insert::new(
        Identifier::new("test"),
        vec![
            ("field1".into(), Box::new(Literal::from("value1"))),
            ("field2".into(), Box::new(Literal::from(132))),
        ],
    );
    dbg!(formatter.formats(&insert2));
}
