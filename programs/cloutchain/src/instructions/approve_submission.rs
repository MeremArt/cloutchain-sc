use anchor_lang::prelude::*;
use crate::state::program_settings::ProgramSettings;
use crate::state::tweet_submission::TweetSubmission;
use crate::state::enums;
use crate::events::tweet_events::SubmissionApprovedEvent;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ApproveSubmission<'info> {
    #[account(mut)]
    pub approver: Signer<'info>,
    #[account(mut)]
    pub tweet_submission: Account<'info, TweetSubmission>,
    #[account(
        seeds = [b"program-settings"],
        bump,
    )]
    pub program_settings: Account<'info, ProgramSettings>,
    pub system_program: Program<'info, System>,
}

pub fn approve_submission(ctx: Context<ApproveSubmission>) -> Result<()> {
    let program_settings = &ctx.accounts.program_settings;
    let submission = &mut ctx.accounts.tweet_submission;

    require!(
        ctx.accounts.approver.key() == program_settings.admin,
        ErrorCode::Unauthorized
    );
    require!(
        submission.status == enums::SubmissionStatus::Pending,
        ErrorCode::InvalidSubmissionStatus
    );

    submission.status = enums::SubmissionStatus::Approved;
    submission.approved_time = Clock::get()?.unix_timestamp;

    let program_settings = &mut ctx.accounts.program_settings;
    program_settings.approved_submissions = program_settings.approved_submissions.checked_add(1).unwrap();

    emit!(SubmissionApprovedEvent {
        submitter: submission.submitter,
        tweet_id: submission.tweet_id.clone(),
        approver: ctx.accounts.approver.key(),
        approved_time: submission.approved_time,
    });

    Ok(())
}
