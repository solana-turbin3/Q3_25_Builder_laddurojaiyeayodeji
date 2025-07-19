use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Amount staked exceeded maximum.")]
    MaxStakeExceeded,
    #[msg("Freeze period not yet met.")]
    InvalidTime,
    #[msg("No points available to claim")]
    NoPointsToClaim,
    #[msg("Invalid claim amount.")]
    InvalidClaimAmount,
}