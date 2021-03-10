#![feature(proc_macro_hygiene)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[program]
pub mod plutocratic_hosting {
    use super::*;

    // TODO action control
    pub fn initialize(ctx: Context<Initialize>, price: u64, content: String, nonce: u8) -> ProgramResult {
        // Transfer funds to the contract vault.
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info().clone(),
            to: ctx.accounts.vault.to_account_info().clone(),
            authority: ctx.accounts.owner.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, price)?;

        // Initialize the content data.
        let content_record = &mut ctx.accounts.content;
        content_record.price = price;
        content_record.content = content;
        content_record.nonce = nonce;
        content_record.owner = *ctx.accounts.from.to_account_info().key;
        content_record.vault = *ctx.accounts.vault.to_account_info().key;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    content: ProgramAccount<'info, ContentRecord>,
    #[account(mut, "&vault.owner == contract_signer.key")]
    vault: CpiAccount<'info, TokenAccount>,
    /// Program derived address for the contract.
    contract_signer: AccountInfo<'info>,
    /// Token account the contract is made from.
    #[account(mut, has_one = owner)]
    from: CpiAccount<'info, TokenAccount>,
    /// Owner of the `from` token account.
    owner: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[account]
pub struct ContentRecord {
    /// Price at which the current content is owned.
    pub price: u64,
    /// Content Data.
    pub content: String,
    /// Public key of current owner of the content.
    pub owner: Pubkey,
    /// Address for token program of funds locked in contract.
    pub vault: Pubkey,
    /// Nonce for the content, to create valid program derived addresses.
    pub nonce: u8,
}
