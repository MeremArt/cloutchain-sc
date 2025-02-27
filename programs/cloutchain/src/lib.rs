use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;
pub mod error;

pub use state::*;
pub use events::*;
pub use error::*;

declare_id!("Fn58PpPQSJHchMySYU8Nb84qAmjWcBU96E5HJ7TtZfB4");

#[program]
pub mod cloutchain {
    use super::*;

    pub fn initialize(
        ctx: Context<instructions::initialize::InitializeProgram>,
        min_stake_amount: u64,
        standard_stake_amount: u64,
        premium_stake_amount: u64,
        admin_fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize::initialize_program(
            ctx,
            min_stake_amount,
            standard_stake_amount,
            premium_stake_amount,
            admin_fee_bps,
        )
    }

    pub fn submit_tweet(
        ctx: Context<instructions::tweet_submission::SubmitTweet>,
        tweet_id: String,
        tweet_url: String,
        stake_tier: state::enums::StakeTier,
        prediction_window_hours: u8,
    ) -> Result<()> {
        instructions::tweet_submission::submit_tweet(
            ctx,
            tweet_id,
            tweet_url,
            stake_tier,
            prediction_window_hours,
        )
    }

    pub fn approve_submission(
        ctx: Context<instructions::approve_submission::ApproveSubmission>,
    ) -> Result<()> {
        instructions::approve_submission::approve_submission(ctx)
    }
}
