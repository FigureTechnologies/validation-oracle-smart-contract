use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{Decimal, OwnedDeps, Response, Uint128};
use provwasm_mocks::ProvenanceMockQuerier;
use provwasm_std::ProvenanceQuery;

pub type MockOwnedDeps = OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery>;

pub fn single_attribute_for_key<'a, T>(response: &'a Response<T>, key: &'a str) -> &'a str {
    response
        .attributes
        .iter()
        .find(|attr| attr.key.as_str() == key)
        .unwrap_or_else(|| panic!("expected to find an attribute with key [{}]", key))
        .value
        .as_str()
}

pub fn decimal(value: u128) -> Decimal {
    Decimal::new(Uint128::new(value))
}
