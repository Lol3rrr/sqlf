use sqlf::{Condition, Select};

fn main() {
    let mut sqlite_formatter = sqlf::fmt::sqlite::SqliteFormatter::new();

    let root_query = Select::new("tests", (), &["first"]);
    dbg!(sqlite_formatter.format_select(&root_query));

    let nested_query = Select::new(
        Select::new("root", (), &["testing"]),
        (),
        vec!["test".to_string()],
    );
    dbg!(sqlite_formatter.format_select(&nested_query));

    let with_condition = Select::new("testing", ("name", "value"), &["first"]);
    dbg!(sqlite_formatter.format_select(&with_condition));

    let with_and_condition =
        Select::new("testing", ("first", "1").and(("second", "2")), &["first"]);
    dbg!(sqlite_formatter.format_select(&with_and_condition));

    let with_or_condition = Select::new("testing", ("first", "1").or(("second", "2")), &["first"]);
    dbg!(sqlite_formatter.format_select(&with_or_condition));
}
