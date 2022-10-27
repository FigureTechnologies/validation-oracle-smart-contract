use crate::types::core::error::ContractError;

pub fn assert_missing_field_error<S: Into<String>>(err: ContractError, expected_missing_fields: S) {
    let expected_missing_fields = expected_missing_fields.into();
    match err {
        ContractError::MissingFields { fields } => {
            assert_eq!(
                expected_missing_fields, fields,
                "the expected missing fields were not specified",
            );
        }
        e => panic!("unexpected error received: {:?}", e),
    };
}
