use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum StakeTier {
    Minimum,
    Standard,
    Premium,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
    Disputed,
    Slashed,
    Completed,
}
