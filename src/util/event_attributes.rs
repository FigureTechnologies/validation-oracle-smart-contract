use std::collections::HashMap;

use super::constants::{
    ASSET_TYPE_KEY, CONTRACT_INFO_KEY, ENTITY_ADDRESSES_KEY, EVENT_TYPE_KEY, NEW_VALUE_KEY,
    RESULTS_SCOPE_ADDRESS_KEY, VALIDATION_REQUEST_ID_KEY, VALIDATION_STATUS_KEY,
    VALIDATION_TYPE_KEY, VALIDATOR_ADDRESS_KEY,
};
use crate::{storage::contract_info::ContractInfo, util::constants::ADDITIONAL_METADATA_KEY};

/// An enum that contains all different event types that can occur throughout the [contract's](crate::contract)
/// routes. Takes strings
#[derive(Clone, Debug)]
pub enum EventType {
    /// Occurs when the contract is [instantiated](crate::contract::instantiate) with [instantiate](crate::instantiate).
    InstantiateContract,
    /// Occurs when the contract is [migrated](crate::contract::migrate) with [migrate](crate::migrate).
    MigrateContract,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [create an entity](crate::execute::entity::create_new_entity).
    AddEntity,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [update an entity](crate::execute::entity::update_existing_entity).
    UpdateEntity,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [create a validation definition](crate::execute::validation_definition::create_new_validation_definition).
    AddValidationDefinition,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [update a validation definition](crate::execute::validation_definition::update_existing_validation_definition).
    UpdateValidationDefinition,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [delete a validation definition](crate::execute::validation_definition::delete_validation_definition).
    DeleteValidationDefinition,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [create a validator configuration](crate::execute::validator_configuration::create_new_validator_configuration).
    AddValidatorConfiguration,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [update a validator configuration](crate::execute::validator_configuration::update_existing_validator_configuration).
    UpdateValidatorConfiguration,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [create a validation request](crate::execute::validation_request::create_request_for_validation).
    AddValidationRequest,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [update a validation request](crate::execute::validation_request::update_request_for_validation).
    UpdateValidationRequest,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [delete a validation request](crate::execute::validation_request::delete_request_for_validation).
    DeleteValidationRequest,
    /// Occurs when the contract is [executed](crate::contract::execute) to
    /// [update its settings](crate::execute::update_settings::update_settings).
    UpdateSettings,
}
#[allow(clippy::from_over_into)]
impl Into<String> for EventType {
    // TODO: Think about Into<String> versus Display (https://stackoverflow.com/q/25316115/)
    fn into(self) -> String {
        match self {
            EventType::InstantiateContract => "instantiate_contract",
            EventType::MigrateContract => "migrate_contract",
            EventType::AddEntity => "add_entity",
            EventType::UpdateEntity => "update_entity",
            EventType::AddValidationDefinition => "add_validation_definition",
            EventType::UpdateValidationDefinition => "update_validation_definition",
            EventType::DeleteValidationDefinition => "delete_validation_definition",
            EventType::AddValidatorConfiguration => "add_validator_configuration",
            EventType::UpdateValidatorConfiguration => "update_validator_configuration",
            EventType::AddValidationRequest => "create_validation_request",
            EventType::UpdateValidationRequest => "update_validation_request",
            EventType::DeleteValidationRequest => "delete_validation_request",
            EventType::UpdateSettings => "update_settings",
        }
        .into()
    }
}
impl EventType {
    /// Utilizes the implementation of `Into<String>` to automatically derive the event name.
    /// This allows an invocation without an explicit type declaration.
    pub fn event_name(self) -> String {
        self.into()
    }
}

/// A helper struct to emit attributes for a [Response](cosmwasm_std::Response).
pub struct EventAttributes {
    /// All generated attributes as tuples, which can easily be used to add into a [Response](cosmwasm_std::Response).
    attributes: Vec<(String, String)>,
}
impl EventAttributes {
    /// Constructs a new instance of this struct.
    ///
    /// # Parameters
    ///
    /// * `event_type` All events should denote their type for external consumers of Provenance
    /// Blockchain Event Stream, so this value is required for any new instance and appends the
    /// name of the event with the key of [EVENT_TYPE_KEY](super::constants::EVENT_TYPE_KEY).
    pub fn new(event_type: EventType) -> Self {
        EventAttributes {
            attributes: vec![(EVENT_TYPE_KEY.into(), event_type.into())],
        }
    }

    /// Appends an asset type value to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `asset_type` An enumerated value for the type of the asset(s) targeted for validation that are
    /// associated with the event, keyed to [ASSET_TYPE_KEY](super::constants::ASSET_TYPE_KEY).
    pub fn set_asset_type<T: Into<String>>(mut self, asset_type: T) -> Self {
        self.attributes
            .push((ASSET_TYPE_KEY.to_string(), asset_type.into()));
        self
    }

    /// Appends a validation request ID to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `validation_request_id` An ID for a validation request associated with the event,
    /// keyed to [VALIDATION_REQUEST_ID_KEY](super::constants::VALIDATION_REQUEST_ID_KEY).
    pub fn set_validation_request_id<T: Into<String>>(mut self, validation_request_id: T) -> Self {
        self.attributes.push((
            VALIDATION_REQUEST_ID_KEY.to_string(),
            validation_request_id.into(),
        ));
        self
    }

    /// Appends a validation status value to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `validation_status` A status for a validation request associated with the event,
    /// keyed to [VALIDATION_STATUS_KEY](super::constants::VALIDATION_STATUS_KEY).
    pub fn set_validation_status<T: Into<String>>(mut self, validation_status: T) -> Self {
        self.attributes
            .push((VALIDATION_STATUS_KEY.to_string(), validation_status.into()));
        self
    }

    /// Appends a validation type value to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `validation_type` An enumerated value for the type of validation associated with the event,
    /// keyed to [VALIDATION_TYPE_KEY](super::constants::VALIDATION_TYPE_KEY).
    pub fn set_validation_type<T: Into<String>>(mut self, validation_type: T) -> Self {
        self.attributes
            .push((VALIDATION_TYPE_KEY.to_string(), validation_type.into()));
        self
    }

    /// Appends a validation results' bech32 scope address to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `scope_address` The bech32 address for the validation results scope associated with the event, keyed to
    /// [RESULTS_SCOPE_ADDRESS_KEY](super::constants::RESULTS_SCOPE_ADDRESS_KEY).
    pub fn set_results_scope_address<T: Into<String>>(mut self, scope_address: T) -> Self {
        self.attributes
            .push((RESULTS_SCOPE_ADDRESS_KEY.to_string(), scope_address.into()));
        self
    }

    // TODO: (1) Change the struct to use a map (2) support more than one validator handling a single request?
    /// Appends a validator's bech32 address to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `validator_address` The bech32 address for the validator associated with the event, keyed to
    /// [VALIDATOR_ADDRESS_KEY](super::constants::VALIDATOR_ADDRESS_KEY).
    pub fn set_validator<T: Into<String>>(mut self, validator_address: T) -> Self {
        self.attributes
            .push((VALIDATOR_ADDRESS_KEY.to_string(), validator_address.into()));
        self
    }

    /// Appends one or more addresses to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `addresses` A collection of addresses of entities associated with the event,
    /// keyed to [ENTITY_ADDRESSES_KEY](super::constants::ENTITY_ADDRESSES_KEY).
    pub fn set_entity_addresses(mut self, addresses: &[String]) -> Self {
        self.attributes
            .push((ENTITY_ADDRESSES_KEY.to_string(), addresses.join(", ")));
        self
    }

    /// Appends a dynamic value to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `new_value` Any dynamic value that pertains to the current execution process, using the
    /// key [NEW_VALUE_KEY](super::constants::NEW_VALUE_KEY).
    pub fn set_new_value<T: ToString>(mut self, new_value: T) -> Self {
        self.attributes
            .push((NEW_VALUE_KEY.to_string(), new_value.to_string()));
        self
    }

    /// Appends a [ContractInfo] to an existing [EventAttributes] and returns
    /// the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `contract_info` An instance of top-level contract information.
    pub fn set_contract_info(mut self, contract_info: &ContractInfo) -> Self {
        self.attributes.push((
            CONTRACT_INFO_KEY.to_string(),
            format!("{:?}", contract_info),
        ));
        self
    }

    /// Appends a dynamic set of additional metadata to an existing [EventAttributes](self::EventAttributes)
    /// and returns the same instance to create a functional chain for further attribute addition.
    /// Note: If the metadata provided is empty, this key will be skipped to prevent strange value
    /// displays to external consumers.
    ///
    /// # Parameters
    ///
    /// * `additional_metadata` An instance of additional metadata to be displayed to any external
    /// consumers.  Uses the key of [ADDITIONAL_METADATA_KEY](super::constants::ADDITIONAL_METADATA_KEY).
    pub fn set_additional_metadata(
        mut self,
        additional_metadata: &EventAdditionalMetadata,
    ) -> Self {
        // Only append additional metadata if it actually has keys
        if additional_metadata.has_metadata() {
            self.attributes.push((
                ADDITIONAL_METADATA_KEY.to_string(),
                additional_metadata.get_meta_string(),
            ));
        }
        self
    }
}

impl IntoIterator for EventAttributes {
    type Item = (String, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

/// A helper collection that allows underlying processes to specify dynamic key values for processes
/// that don't necessarily need to specify a large amount of new event keys.  Emitted values are
/// aggregated and sorted deterministically, and then displayed using the format:
/// \[a=1, b=2, c=3, etc\]
pub struct EventAdditionalMetadata {
    /// An internal collection of all added metadata.
    fields: HashMap<String, String>,
}
impl EventAdditionalMetadata {
    /// Constructs a new instance of this struct with an empty fields set.
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Returns `true` only if metadata fields have been added with the [add_metadata](self::EventAdditionalMetadata::add_metadata)
    /// function.
    pub fn has_metadata(&self) -> bool {
        !self.fields.is_empty()
    }

    /// Appends a new key and value pair to the internal fields value.
    ///
    /// # Parameters
    ///
    /// * `key` The string key that will be displayed before the = sign in the display.
    /// * `value` The string value that will be displayed after the = sign in the display.
    pub fn add_metadata<S1: Into<String>, S2: Into<String>>(&mut self, key: S1, value: S2) {
        self.fields.insert(key.into(), value.into());
    }

    /// Aggregates and deterministically sorts the internal values, resulting in a display string
    /// for adding as an event attribute in the format: \[a=1, b=2, c=3, etc\]
    pub fn get_meta_string(&self) -> String {
        let mut map_displays = self
            .fields
            .iter()
            .map(|(key, value)| format!("[{key}={value}]"))
            .collect::<Vec<_>>();
        // Keep the collection sorted to ensure that output is deterministic
        map_displays.sort();
        map_displays.join(", ")
    }
}
impl Default for EventAdditionalMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{EventAttributes, EventType};
    use crate::test::helpers::single_attribute_for_key;
    use crate::util::constants::{
        ASSET_TYPE_KEY, EVENT_TYPE_KEY, NEW_VALUE_KEY, RESULTS_SCOPE_ADDRESS_KEY,
        VALIDATION_TYPE_KEY, VALIDATOR_ADDRESS_KEY,
    };
    use crate::util::event_attributes::EventAdditionalMetadata;

    use cosmwasm_std::Response;

    #[test]
    fn test_response_consumption() {
        let attributes = EventAttributes::new(EventType::AddValidationDefinition)
            .set_asset_type("asset type")
            .set_validation_type("validation type")
            .set_results_scope_address("results scope address")
            .set_validator("validator address")
            .set_new_value("new value");
        let response: Response<String> = Response::new().add_attributes(attributes);
        assert_eq!(
            "add_validation_definition",
            single_attribute_for_key(&response, EVENT_TYPE_KEY),
            "the event type attribute should be added correctly",
        );
        assert_eq!(
            "asset type",
            single_attribute_for_key(&response, ASSET_TYPE_KEY),
            "the asset type attribute should be added correctly",
        );
        assert_eq!(
            "validation type",
            single_attribute_for_key(&response, VALIDATION_TYPE_KEY),
            "the validation type attribute should be added correctly",
        );
        assert_eq!(
            "results scope address",
            single_attribute_for_key(&response, RESULTS_SCOPE_ADDRESS_KEY),
            "the results scope address attribute should be added correctly",
        );
        assert_eq!(
            "validator address",
            single_attribute_for_key(&response, VALIDATOR_ADDRESS_KEY),
            "the validator address attribute should be added correctly",
        );
        assert_eq!(
            "new value",
            single_attribute_for_key(&response, NEW_VALUE_KEY),
            "the new value attribute should be added correctly",
        );
    }

    #[test]
    fn test_additional_metadata_string_output() {
        let mut metadata = EventAdditionalMetadata::new();
        assert_eq!(
            "",
            metadata.get_meta_string(),
            "expected no output to be derived when no metadata has been added",
        );
        metadata.add_metadata("b", "b_value");
        assert_eq!(
            "[b=b_value]",
            metadata.get_meta_string(),
            "expected the key/value addition to display properly",
        );
        metadata.add_metadata("a", "a_value");
        assert_eq!(
            "[a=a_value], [b=b_value]",
            metadata.get_meta_string(),
            "expected the second key/value addition to also display alongside the first, alphabetically sorted",
        );
        metadata.add_metadata("c", "c_value");
        assert_eq!(
            "[a=a_value], [b=b_value], [c=c_value]",
            metadata.get_meta_string(),
            "expected the third key/value addition to also display alongside the first two, alphabetically sorted",
        );
    }
}
