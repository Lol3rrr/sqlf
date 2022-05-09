use sqlf::{
    fmt::{Formatter, SqliteBackend},
    Delete, Identifier, Literal,
};

fn main() {
    let formatter = Formatter::new(SqliteBackend::new());

    let delete_without_pred = Delete::new(Identifier::new("test"), ());
    dbg!(formatter.formats(&delete_without_pred));

    let delete_with_pred = Delete::new(
        Identifier::new("test"),
        (Identifier::new("field"), Literal::from("value")),
    );
    dbg!(formatter.formats(&delete_with_pred));
}
