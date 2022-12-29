use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Fundraiser Name is too long")]
    NameTooLong,
    #[msg("Description is too long")]
    DescriptionTooLong,
    #[msg("Invalid Authority to Update")]
    InvalidAuthority,
    #[msg("Attempting to withdraw more than Fundraiser's balance")]
    AmountTooLarge,
    #[msg("Fundraiser's goal has not been met")]
    GoalNotMet,
    #[msg("Invalid Beneficiary provided")]
    InvalidBeneficiary,
}
