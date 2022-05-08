use sqlf::{Identifier, Literal, Predicate, Select};

fn main() {
    let formatter = sqlf::fmt::Formatter::new(sqlf::fmt::SqliteBackend::new());

    let root_query = Select::new(
        Identifier::from("test"),
        vec![Identifier::from("first")],
        (),
    );
    dbg!(formatter.formatq(&root_query));

    let nested_query = Select::new(
        Select::new(Identifier::from("base"), vec!["testing".into()], ()),
        vec!["test".into()],
        (),
    );
    dbg!(formatter.formatq(&nested_query));

    let with_condition = Select::new(
        Identifier::from("testing"),
        vec!["first".into()],
        (Identifier::from("name"), Literal::from("value")),
    );
    dbg!(formatter.formatq(&with_condition));

    let with_and_condition = Select::new(
        Identifier::from("testing"),
        vec!["first".into()],
        (Identifier::from("first"), Literal::from("1"))
            .and((Identifier::from("second"), Literal::from("2"))),
    );
    dbg!(formatter.formatq(&with_and_condition));

    let with_or_condition = Select::new(
        Identifier::from("testing"),
        vec!["first".into()],
        (Identifier::from("first"), Literal::from("1"))
            .or((Identifier::from("second"), Literal::from("2"))),
    );
    dbg!(formatter.formatq(&with_or_condition));
}
