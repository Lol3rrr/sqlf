use sqlf::{
    fmt::{Formatter, SqliteBackend},
    Identifier, Order, Select,
};

#[test]
fn select() {
    let formatter = Formatter::new(SqliteBackend::new());

    let select = Select::new(Identifier::new("test"), vec!["test".into()], ());

    let result = formatter.formatq(&select);

    assert_eq!("SELECT test FROM (test)", result.to_string());
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

    assert_eq!(
        "SELECT test FROM (SELECT test,testing FROM (inner))",
        result.to_string()
    );
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

    assert_eq!(
        "SELECT test FROM (test) WHERE test=\"value\"",
        result.to_string()
    );
}

#[test]
fn ordering_select() {
    let formatter = Formatter::new(SqliteBackend::new());

    let select = Select::new(Identifier::new("test"), vec!["test".into()], ())
        .order(Identifier::new("other"), Order::Descending);

    let result = formatter.formatq(&select);

    assert_eq!(
        "SELECT test FROM (test) ORDER BY other DESC",
        result.to_string()
    );
}
