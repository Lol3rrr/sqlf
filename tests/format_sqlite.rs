use sqlf::Select;

#[test]
fn select() {
    let mut formatter = sqlf::fmt::sqlite::SqliteFormatter::new();

    let select = Select::new("test", (), &["test"]);

    let result = formatter.format_select(&select);
    dbg!(result);
}

#[test]
fn nested_select() {
    let mut formatter = sqlf::fmt::sqlite::SqliteFormatter::new();

    let select = Select::new(
        Select::new("inner", (), &["test", "testing"]),
        (),
        &["test"],
    );

    let result = formatter.format_select(&select);
    dbg!(result);
}

#[test]
fn conditional_select() {
    let mut formatter = sqlf::fmt::sqlite::SqliteFormatter::new();

    let select = Select::new("test", ("test", "value"), &["test"]);

    let result = formatter.format_select(&select);
    dbg!(result);
}
