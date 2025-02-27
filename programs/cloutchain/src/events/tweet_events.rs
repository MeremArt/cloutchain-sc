use anchor_lang::prelude::*;
use crate::enums::StakeTier;

#[event]
pub struct TweetSubmittedEvent {
    pub submitter: Pubkey,
    pub tweet_id: String,
    pub stake_amount: u64,
    pub stake_tier: StakeTier,
    pub submission_time: i64,
    pub expiry_time: i64,
}

#[event]
pub struct SubmissionApprovedEvent {
    pub submitter: Pubkey,
    pub tweet_id: String,
    pub approver: Pubkey,
    pub approved_time: i64,
}
