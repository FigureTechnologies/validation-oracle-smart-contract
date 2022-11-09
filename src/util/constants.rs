///// Global constants

/// The [Coin](cosmwasm_std::Coin) denomination corresponding to one-billionth of a single hash.
pub const NHASH: &str = "nhash";

///// Shared output attributes

// TODO: Briefly discuss if we want to keep the prefixes

/// Value = Event Type correlating to EvenType enum into [String] values.
pub const EVENT_TYPE_KEY: &str = "vo_event_type";
/// Value = Scope UUID (String) of validation results.
pub const RESULTS_SCOPE_ADDRESS_KEY: &str = "vo_results_scope_address";
/// Value = Validation Type (String).
pub const VALIDATION_TYPE_KEY: &str = "vo_validation_type";
/// Value = Asset Type (String).
pub const ASSET_TYPE_KEY: &str = "vo_asset_type";
/// Value = The address of the associated validator.
pub const VALIDATOR_ADDRESS_KEY: &str = "vo_validator_address";
/// Value = The [validation status]() of a [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder) after performing an execute function.
pub const VALIDATION_STATUS_KEY: &str = "vo_validation_request_status";
/// Value = Any new value being changed that can be coerced to a string target.
pub const NEW_VALUE_KEY: &str = "vo_new_value";
/// Value = The output string of an [EventAdditionalMetadata](crate::util::event_attributes::EventAdditionalMetadata).
pub const ADDITIONAL_METADATA_KEY: &str = "vo_additional_metadata";

///// Other output attributes

/// Value = The current [contract state](crate::storage::contract_info::ContractInfo)
pub const CONTRACT_INFO_KEY: &str = "vo_contract_info";
