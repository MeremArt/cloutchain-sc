use anchor_lang::prelude::*;

#[account]
pub struct ProgramSettings {
    pub admin: Pubkey,
    pub min_stake_amount: u64,
    pub standard_stake_amount: u64,
    pub premium_stake_amount: u64,
    pub admin_fee_bps: u16,
    pub is_paused: bool,
    pub total_submissions: u64,
    pub approved_submissions: u64,
    pub rejected_submissions: u64,
    pub total_stakes: u64,
    pub total_slashed: u64,
}
