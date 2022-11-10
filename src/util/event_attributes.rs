use super::constants::{
    ASSET_TYPE_KEY, CONTRACT_INFO_KEY, EVENT_TYPE_KEY, NEW_VALUE_KEY, RESULTS_SCOPE_ADDRESS_KEY,
    VALIDATION_REQUEST_ID_KEY, VALIDATION_STATUS_KEY, VALIDATION_TYPE_KEY, VALIDATOR_ADDRESS_KEY,
};
use crate::{
    storage::contract_info::ContractInfo,
    types::request::validation_request::ValidationRequestOrder,
    util::constants::ADDITIONAL_METADATA_KEY,
};

use std::collections::HashMap;

/// An enum that contains all different event types that can occur throughout the [contract's](crate::contract)
/// routes.
#[derive(Clone, Debug)]
pub enum EventType<'a> {
    /// Occurs when the contract is [instantiated](crate::contract::instantiate) with [instantiate](crate::instantiate).
    InstantiateContract(&'a ContractInfo),
    /// Occurs when the contract is [migrated](crate::contract::migrate) with [migrate](crate::migrate).
    MigrateContract,
    /// Occurs when the contract is [executed](crate::contract::execute) to [create a validation definition](crate::execute::create_validation_definition).
    AddValidationDefiniton,
    /// Occurs when the contract is [executed](crate::contract::execute) to [create a validation request](crate::execute::create_request).
    AddValidationRequest(&'a ValidationRequestOrder),
}
#[allow(clippy::from_over_into)]
impl Into<String> for EventType<'_> {
    // TODO: Think about Into<String> versus Display
    fn into(self) -> String {
        match self {
            EventType::InstantiateContract(_) => "instantiate_contract",
            EventType::MigrateContract => "migrate_contract",
            EventType::AddValidationDefiniton => "add_validation_definition",
            EventType::AddValidationRequest(_) => "create_validation_request",
        }
        .into()
    }
}
impl EventType<'_> {
    /// Utilizes the implementation of Into<String> to automatically derive the event name.  This
    /// allows an invocation without an explicit type declaration.
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
        let mut attributes = vec![(EVENT_TYPE_KEY.into(), event_type.clone().event_name())];
        let maybe_associated_attribute = match event_type {
            // TODO: Does this match have to return a vec instead of an array?
            EventType::InstantiateContract(contract_info) => {
                Some([(CONTRACT_INFO_KEY.into(), format!("{:?}", contract_info))].to_vec())
            }
            EventType::MigrateContract => None,
            EventType::AddValidationDefiniton => None,
            EventType::AddValidationRequest(request) => Some(
                [
                    (VALIDATION_REQUEST_ID_KEY.into(), request.get_id().into()),
                    (VALIDATION_STATUS_KEY.into(), request.status.to_string()),
                ]
                .to_vec(),
            ),
        };
        if let Some(associated_attribute) = maybe_associated_attribute {
            attributes.extend_from_slice(&associated_attribute);
        }
        EventAttributes { attributes }
    }

    // TODO: Change link in line below to validation results submission method

    /// Certain contract events like [create_request](crate::execute::create_request::create_request_for_validation)
    /// benefit from having a standardized set of event types to facilitate processing them from the event stream.
    /// This is a constructor for a struct that includes those values to facilitate the process of generating
    /// all the relevant attributes.
    ///
    /// # Parameters
    ///
    /// * `event_type` All events should denote their type for external consumers of the Provenance
    /// Blockchain Event Stream, so this value is required for any new instance and appends the
    /// name of the event with the key of [EVENT_TYPE_KEY](super::constants::EVENT_TYPE_KEY).
    /// * `asset_type` An enumerated value for the type of the asset(s) targeted for validation that are
    /// associated with the event, keyed to [ASSET_TYPE_KEY](super::constants::ASSET_TYPE_KEY).
    /// * `validation_type` An enumerated value for the type of validation associated with the event,
    /// keyed to [VALIDATION_TYPE_KEY](super::constants::VALIDATION_TYPE_KEY).
    /// * `scope_address` The bech32 address for the validation results scope associated with the event, keyed to
    /// [RESULTS_SCOPE_ADDRESS_KEY](super::constants::RESULTS_SCOPE_ADDRESS_KEY).
    pub fn for_results_submission<T1: Into<String>, T2: Into<String>, T3: Into<String>>(
        event_type: EventType,
        asset_type: T1,
        validation_type: T2,
        scope_address: T3,
    ) -> Self {
        Self::new(event_type)
            .set_asset_type(asset_type)
            .set_validation_type(validation_type)
            .set_results_scope_address(scope_address)
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
            .push((ASSET_TYPE_KEY.into(), asset_type.into()));
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
            .push((VALIDATION_TYPE_KEY.into(), validation_type.into()));
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
            .push((RESULTS_SCOPE_ADDRESS_KEY.into(), scope_address.into()));
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
            .push((VALIDATOR_ADDRESS_KEY.into(), validator_address.into()));
        self
    }

    /// Appends a validation request's status value to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    /// * `status` The onboarding status of the current
    /// [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder)
    /// associated with the given event.
    // pub fn set_new_validation_status(mut self, status: &ValidationRequestStatus) -> Self {
    //     self.attributes
    //         .push((VALIDATION_STATUS_KEY.into(), status.to_string()));
    //     self
    // }

    /// Appends a dynamic value to an existing [EventAttributes](self::EventAttributes) and
    /// returns the same instance to create a functional chain for further attribute addition.
    ///
    /// # Parameters
    ///
    /// * `new_value` Any dynamic value that pertains to the current execution process, using the
    /// key [NEW_VALUE_KEY](super::constants::NEW_VALUE_KEY).
    pub fn set_new_value<T: ToString>(mut self, new_value: T) -> Self {
        self.attributes
            .push((NEW_VALUE_KEY.into(), new_value.to_string()));
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
                ADDITIONAL_METADATA_KEY.into(),
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
    use cosmwasm_std::Response;

    use crate::test::cosmos_type_helpers::single_attribute_for_key;
    use crate::util::constants::{
        ASSET_TYPE_KEY, EVENT_TYPE_KEY, NEW_VALUE_KEY, RESULTS_SCOPE_ADDRESS_KEY,
        VALIDATION_TYPE_KEY, VALIDATOR_ADDRESS_KEY,
    };
    use crate::util::event_attributes::EventAdditionalMetadata;

    use super::{EventAttributes, EventType};

    #[test]
    fn test_response_consumption() {
        let attributes = EventAttributes::new(EventType::AddValidationDefiniton)
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
