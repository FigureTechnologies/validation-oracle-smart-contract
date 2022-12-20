use cosmwasm_std::StdError;
use thiserror::Error;

/// An enumeration of all the possible errors that may be returned from any of
/// the contract's [entrypoints](crate::contract).
// TODO: Rename parameters to clearly indicate if their values should be capitalized or not
#[derive(Error, Debug)]
pub enum ContractError {
    // TODO: Consolidate usage of ExistingId and RecordAlreadyExists to correctly use either of the two across the board
    /// A generic error returned from attempting a contract operation
    /// which requires a unique identifier with a duplicate.
    #[error("Cannot use [{id_type}] with id [{id}]. One with that id already exists")]
    ExistingId { id_type: String, id: String },

    /// A completely generic error that can be returned from anywhere in the
    /// contract when a more specific & applicable variant is not defined.
    #[error("{message}")]
    GenericError { message: String },

    /// A generic error returned from contract [instantiation](crate::contract::instantiate).
    #[error("Invalid instantation: {message}")]
    InvalidInstantiation { message: String },

    /// An error to specify that some form of provided or utilized coin was invalid.
    #[error("Invalid funds: {message}")]
    InvalidFunds {
        /// Denotes the reason that invalid funds were detected.
        message: String,
    },

    // TODO: Consolidate usage of InvalidRequest versus RecordNotFound for update operations
    /// A generic error returned from a contract entrypoint when a more specific & applicable
    /// variant for indicating some problem with the request input is not defined.
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },

    /// A generic error returned from contract [migration](crate::contract::migrate) when a more specific
    /// & applicable variant for indicating some problem with the request input is not defined.
    #[error("Invalid migration: {message}")]
    InvalidMigration { message: String },

    /// A generic error for when an invalid type is provided to a method call. This error generally
    /// should not be returned from contract entrypoints.
    #[error("Invalid type encountered: {explanation}")]
    InvalidType { explanation: String },

    /// A generic error returned from a contract [execution](crate::contract::execute) when a more specific
    /// & applicable variant for indicating some problem performing the requested update is not defined.
    #[error("Invalid update: {explanation}")]
    InvalidUpdate { explanation: String },

    /// An error returned from a contract entrypoint indicating that
    /// certain required fields are missing from the input.
    #[error("Missing fields: {fields:?}")]
    MissingFields { fields: String },

    /// An error that occurs when a unique key is violated during an attempt to
    /// add new data to the contract's internal [storage](crate::storage).
    #[error("Existing record found: {explanation}")]
    RecordAlreadyExists { explanation: String }, // TODO: Probably just remove this type in favor of InvalidRequest

    /// An error returned when a mandatory data lookup is performed on the contract's
    /// internal [storage](crate::storage), but the required value is not found.
    #[error("{explanation}")]
    RecordNotFound { explanation: String },

    /// A wrapper for a [semver Error](semver::Error).
    #[error("{0}")]
    SemVerError(#[from] semver::Error),

    /// A wrapper for [StdError].
    #[error("{0}")]
    Std(#[from] StdError),

    /// An error returned from internal [storage](crate::storage) operations.
    #[error("Contact storage error occurred: {message}")]
    StorageError { message: String },

    /// An error returned from a contract entrypoint when the requestor is not
    /// permitted to perform the requested operation for any reason.
    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },
}
