use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::program_settings::ProgramSettings;
use crate::state::tweet_submission::TweetSubmission;
use crate::state::enums;
use crate::events::tweet_events::TweetSubmittedEvent;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct SubmitTweet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<TweetSubmission>() + 150,
        seeds = [b"tweet-submission", tweet_id.as_bytes()],
        bump
    )]
    pub tweet_submission: Account<'info, TweetSubmission>,
    #[account(
        seeds = [b"program-settings"],
        bump,
    )]
    pub program_settings: Account<'info, ProgramSettings>,
    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn submit_tweet(
    ctx: Context<SubmitTweet>,
    tweet_id: String,
    tweet_url: String,
    stake_tier: enums::StakeTier,
    prediction_window_hours: u8,
) -> Result<()> {
    let program_settings = &ctx.accounts.program_settings;
    require!(!program_settings.is_paused, ErrorCode::ProgramPaused);
    require!(
        prediction_window_hours >= 24 && prediction_window_hours <= 168,
        ErrorCode::InvalidPredictionWindow
    );

    let stake_amount = match stake_tier {
        enums::StakeTier::Minimum => program_settings.min_stake_amount,
        enums::StakeTier::Standard => program_settings.standard_stake_amount,
        enums::StakeTier::Premium => program_settings.premium_stake_amount,
    };

    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, stake_amount)?;

    let submission = &mut ctx.accounts.tweet_submission;
    submission.submitter = ctx.accounts.user.key();
    submission.tweet_id = tweet_id;
    submission.tweet_url = tweet_url;
    submission.stake_amount = stake_amount;
    submission.stake_tier = stake_tier;
    submission.status = enums::SubmissionStatus::Pending;
    submission.submission_time = Clock::get()?.unix_timestamp;
    let prediction_window_secs = (prediction_window_hours as i64) * 3600;
    submission.expiry_time = submission.submission_time + prediction_window_secs;
    submission.approved_time = 0;
    submission.resolved_time = 0;
    submission.is_manipulated = false;
    submission.dispute_count = 0;

    let program_settings = &mut ctx.accounts.program_settings;
    program_settings.total_submissions = program_settings.total_submissions.checked_add(1).unwrap();
    program_settings.total_stakes = program_settings.total_stakes.checked_add(stake_amount).unwrap();

    emit!(TweetSubmittedEvent {
        submitter: submission.submitter,
        tweet_id: submission.tweet_id.clone(),
        stake_amount,
        stake_tier,
        submission_time: submission.submission_time,
        expiry_time: submission.expiry_time,
    });

    Ok(())
}
