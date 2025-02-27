use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use std::mem::size_of;

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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
