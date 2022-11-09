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

    /// A generic error that specifies that some form of provided or utilized coin was invalid.
    #[error("Invalid funds: {message}")]
    InvalidFunds {
        /// Denotes the reason that invalid funds were detected.
        message: String,
    },

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

    /// An error that occurs when a unique key is violated during an attempt to add new data to the
    /// contract's internal storage.  Reference: [storage](crate::storage).
    #[error("Existing record found: {explanation}")]
    RecordAlreadyExists {
        /// A free-form text description of the reason that the record already exists.
        explanation: String,
    },

    /// Occurs when a mandatory data lookup is performed on the contract's internal storage, but
    /// the required value is not found.  Reference: [storage](crate::storage).
    #[error("Record not found: {explanation}")]
    RecordNotFound {
        /// A free-form text description of the record that could not be found.
        explanation: String,
    },

    #[error("{0}")]
    SemVerError(#[from] semver::Error),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Contact storage error occurred: {message}")]
    StorageError { message: String },

    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },
}
