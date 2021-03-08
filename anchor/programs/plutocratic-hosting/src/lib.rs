#![feature(proc_macro_hygiene)]

use anchor_lang::prelude::*;

#[program]
pub mod plutocratic_hosting {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}