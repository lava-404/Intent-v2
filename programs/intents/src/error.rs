use anchor_lang::prelude::*;

#[error_code]
pub enum IntentError {
    #[msg("Token amount must be greater than zero")]
    InvalidAmount,

    #[msg("Minimum output must be greater than zero")]
    InvalidMinOutput,

    #[msg("Expiry must be after creation time")]
    InvalidExpiry,

    #[msg("Intent already expired")]
    IntentExpired,

    #[msg("Invalid intent status")]
    InvalidStatus,

    #[msg("Protocol paused")]
    ProtocolPaused,

    #[msg("Invalid Recipient")]
    InvalidRecipient,

    #[msg("Incorrect Owner")]
    IncorrectOwner,

    #[msg("Intent Not Open Anymore")]
    IntentNotOpen,

}
