use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct EntityDetail {
    pub address: Addr,
    pub name: Option<String>,
    pub description: Option<String>,
    pub home_url: Option<String>,
    pub source_url: Option<String>,
}
impl EntityDetail {
    pub fn maybe_get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_ref().unwrap()
    }
    pub fn maybe_get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn get_description(&self) -> &str {
        self.description.as_ref().unwrap()
    }
    pub fn maybe_get_home_url(&self) -> Option<&str> {
        self.home_url.as_deref()
    }
    pub fn get_home_url(&self) -> &str {
        self.home_url.as_ref().unwrap()
    }
    pub fn maybe_get_source_url(&self) -> Option<&str> {
        self.source_url.as_deref()
    }
    pub fn get_source_url(&self) -> &str {
        self.source_url.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::arbitrary::arb_addr;
    use crate::types::entity::EntityDetail;

    use proptest::option::of as option_of;
    use proptest::{prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn set_and_get_entity(
            address in arb_addr(),
            name in option_of(".+"),
            description in option_of(".+"),
            home_url in option_of(".+"),
            source_url in option_of(".+"),
        ) {
            let entity = EntityDetail {
                address: address.clone(),
                name: name.clone(),
                description: description.clone(),
                home_url: home_url.clone(),
                source_url: source_url.clone()
            };
            prop_assert_eq!(address, entity.address.clone());
            prop_assert_eq!(name, entity.maybe_get_name().map(|v| v.to_string()));
            prop_assert_eq!(description, entity.maybe_get_description().map(|v| v.to_string()));
            prop_assert_eq!(home_url, entity.maybe_get_home_url().map(|v| v.to_string()));
            prop_assert_eq!(source_url, entity.maybe_get_source_url().map(|v| v.to_string()));
        }
    }
}
