use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Cannot create [{id_type}] with id [{id}]. One with that id already exists")]
    ExistingId { id_type: String, id: String },

    #[error("{message}")]
    GenericError { message: String },

    #[error("Invalid instantation: {message}")]
    InvalidInstantiation { message: String },

    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },

    #[error("Invalid migration: {message}")]
    InvalidMigration { message: String },

    #[error("Invalid type encountered: {explanation}")]
    InvalidType { explanation: String },

    #[error("Invalid update: {explanation}")]
    InvalidUpdate { explanation: String },

    #[error("Missing fields: {fields:?}")]
    MissingFields { fields: String },

    #[error("{0}")]
    SemVerError(#[from] semver::Error),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Contact storage error occurred: {message}")]
    StorageError { message: String },

    #[error("Unauthorized")]
    Unauthorized,
}
