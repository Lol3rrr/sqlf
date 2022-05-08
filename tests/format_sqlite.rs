use sqlf::{
    fmt::{Formatter, SqliteBackend},
    Identifier, Select,
};

#[test]
fn select() {
    let formatter = Formatter::new(SqliteBackend::new());

    let select = Select::new(Identifier::new("test"), vec!["test".into()], ());

    let result = formatter.formatq(&select);
    dbg!(result);
}

#[test]
fn nested_select() {
    let formatter = Formatter::new(SqliteBackend::new());

    let select = Select::new(
        Select::new(
            Identifier::new("inner"),
            vec!["test".into(), "testing".into()],
            (),
        ),
        vec!["test".into()],
        (),
    );

    let result = formatter.formatq(&select);
    dbg!(result);
}

#[test]
fn conditional_select() {
    let formatter = Formatter::new(SqliteBackend::new());

    let select = Select::new(
        Identifier::new("test"),
        vec!["test".into()],
        ("test".into(), "value".into()),
    );

    let result = formatter.formatq(&select);
    dbg!(result);
}
