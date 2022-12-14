///// Global constants

/// The [Coin](cosmwasm_std::Coin) denomination corresponding to one-billionth of a single hash.
pub const NHASH: &str = "nhash";

///// Shared output attributes

/// Value = Event Type correlating to EvenType enum into [String] values.
pub const EVENT_TYPE_KEY: &str = "vo_event_type";
/// Value = Scope UUID (String) of validation results.
pub const RESULTS_SCOPE_ADDRESS_KEY: &str = "vo_results_scope_address";
/// Value = Asset Type (String).
pub const ASSET_TYPE_KEY: &str = "vo_asset_type";
/// Value = Validation Type (String).
pub const VALIDATION_TYPE_KEY: &str = "vo_validation_type";
/// Value = The address of the associated validator.
pub const VALIDATOR_ADDRESS_KEY: &str = "vo_validator_address";
/// Value = The ID of a [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder).
pub const VALIDATION_REQUEST_ID_KEY: &str = "vo_validation_request_id";
/// Value = The [validation status](crate::types::request::validation_request::ValidationRequestStatus) of a
/// [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder).
pub const VALIDATION_STATUS_KEY: &str = "vo_validation_request_status";
/// Value = Any new value being changed that can be coerced to a string target.
pub const NEW_VALUE_KEY: &str = "vo_new_value";
/// Value = The output string of an [EventAdditionalMetadata](crate::util::event_attributes::EventAdditionalMetadata).
pub const ADDITIONAL_METADATA_KEY: &str = "vo_additional_metadata";

///// Other output attributes

/// Value = The current stored [ContractInfo](crate::storage::contract_info::ContractInfo).
pub const CONTRACT_INFO_KEY: &str = "vo_contract_info";

/// Value = The [address](cosmwasm_std::Addr) of one or more entities.
pub const ENTITY_ADDRESSES_KEY: &str = "vo_entity_addresses";
