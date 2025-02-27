use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Fee percentage is too high")]
    FeeTooHigh,
    #[msg("Program is currently paused")]
    ProgramPaused,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Invalid submission status for this operation")]
    InvalidSubmissionStatus,
    #[msg("Prediction window must be between 24 and 168 hours")]
    InvalidPredictionWindow,
}