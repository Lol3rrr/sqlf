use sqlf::verify::VerifyError;

#[test]
fn valid_select() {
    let table_def = {
        let mut tmp = sqlf::verify::RootTableDefinitions::new();

        tmp.add_table(
            "test",
            ["first", "second"].iter().map(|s| s.to_string()).collect(),
        );

        tmp
    };

    let query = sqlf::Select::new("test", (), &["first"]);

    let verify_result = sqlf::verify::verify_select(query, &table_def);

    assert!(verify_result.is_ok());
}

#[test]
fn missing_field_select() {
    let table_def = {
        let mut tmp = sqlf::verify::RootTableDefinitions::new();

        tmp.add_table(
            "test",
            ["first", "second"].iter().map(|s| s.to_string()).collect(),
        );

        tmp
    };

    let query = sqlf::Select::new("test", (), &["other"]);

    let verify_result = sqlf::verify::verify_select(query, &table_def);

    assert!(verify_result.is_err());
    assert_eq!(
        VerifyError::MissingField {
            field: "other".to_string()
        },
        verify_result.unwrap_err()
    );
}

#[test]
fn missing_table_select() {
    let table_def = {
        let mut tmp = sqlf::verify::RootTableDefinitions::new();

        tmp.add_table(
            "test",
            ["first", "second"].iter().map(|s| s.to_string()).collect(),
        );

        tmp
    };

    let query = sqlf::Select::new("other", (), &["first"]);

    let verify_result = sqlf::verify::verify_select(query, &table_def);

    assert!(verify_result.is_err());
    assert_eq!(
        VerifyError::UnknownTable {
            table: "other".to_string(),
        },
        verify_result.unwrap_err()
    );
}
