use anchor_lang::prelude::*;
use crate::state::enums::{StakeTier, SubmissionStatus};

#[account]
pub struct TweetSubmission {
    pub submitter: Pubkey,
    pub tweet_id: String,
    pub tweet_url: String,
    pub stake_amount: u64,
    pub stake_tier: StakeTier,
    pub status: SubmissionStatus,
    pub submission_time: i64,
    pub expiry_time: i64,
    pub approved_time: i64,
    pub resolved_time: i64,
    pub is_manipulated: bool,
    pub dispute_count: u8,
}
