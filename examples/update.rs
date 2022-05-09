use sqlf::{
    fmt::{Formatter, SqliteBackend},
    Identifier, Literal, Update,
};

fn main() {
    let formatter = Formatter::new(SqliteBackend::new());

    let update_no_pred = Update::new(
        Identifier::new("test"),
        vec![(Identifier::new("field"), Box::new(Literal::from("value")))],
        (),
    );
    dbg!(formatter.formats(&update_no_pred));

    let update_with_pred = Update::new(
        Identifier::new("test"),
        vec![(
            Identifier::new("other"),
            Box::new(Literal::from("New-Value")),
        )],
        (Identifier::new("field"), Literal::from("value")),
    );
    dbg!(formatter.formats(&update_with_pred));
}
