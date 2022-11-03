pub fn generate_validation_definition_attribute_name<T: Into<String>, U: Into<String>>(
    validation_type: T,
    base_contract_name: U,
) -> String {
    format!("{}.{}", validation_type.into(), base_contract_name.into())
}
