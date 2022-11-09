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
