use anchor_lang::prelude::*;
use crate::state::program_settings::ProgramSettings;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + std::mem::size_of::<ProgramSettings>(),
        seeds = [b"program-settings"],
        bump
    )]
    pub program_settings: Account<'info, ProgramSettings>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_program(
    ctx: Context<InitializeProgram>,
    min_stake_amount: u64,
    standard_stake_amount: u64,
    premium_stake_amount: u64,
    admin_fee_bps: u16,
) -> Result<()> {
    require!(admin_fee_bps <= 1000, ErrorCode::FeeTooHigh);

    let program_settings = &mut ctx.accounts.program_settings;
    program_settings.admin = ctx.accounts.admin.key();
    program_settings.min_stake_amount = min_stake_amount;
    program_settings.standard_stake_amount = standard_stake_amount;
    program_settings.premium_stake_amount = premium_stake_amount;
    program_settings.admin_fee_bps = admin_fee_bps;
    program_settings.is_paused = false;
    program_settings.total_submissions = 0;
    program_settings.approved_submissions = 0;
    program_settings.rejected_submissions = 0;
    program_settings.total_stakes = 0;
    program_settings.total_slashed = 0;
    
    Ok(())
}
