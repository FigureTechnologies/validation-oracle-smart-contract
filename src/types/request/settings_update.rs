use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct SettingsUpdate {
    pub new_admin_address: Option<String>,
}
