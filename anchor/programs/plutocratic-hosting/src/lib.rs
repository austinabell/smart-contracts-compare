#![feature(proc_macro_hygiene)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[program]
pub mod plutocratic_hosting {
    use super::*;

    /// Initialize a new contract with initialized content.
    #[access_control(Initialize::validate(&ctx, nonce))]
    pub fn initialize(
        ctx: Context<Initialize>,
        price: u64,
        content: String,
        nonce: u8,
    ) -> ProgramResult {
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

    /// Purchase content address for new price, if transferring more tokens.
    #[access_control(check_funds(&ctx.accounts.content, price))]
    pub fn purchase(ctx: Context<Purchase>, price: u64, content: String) -> ProgramResult {
        let seeds = &[
            ctx.accounts.content.to_account_info().key.as_ref(),
            &[ctx.accounts.content.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info().clone(),
            to: ctx.accounts.owner_token.to_account_info().clone(),
            authority: ctx.accounts.contract_signer.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, ctx.accounts.content.price)?;

        // Overwrite content
        let content_record = &mut ctx.accounts.content;
        content_record.price = price;
        content_record.content = content;
        content_record.owner = *ctx.accounts.new_owner.to_account_info().key;

        Ok(())
    }
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

impl<'info> Initialize<'info> {
    pub fn validate(ctx: &Context<Self>, nonce: u8) -> Result<()> {
        let signer = Pubkey::create_program_address(
            &[
                ctx.accounts.content.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| ErrorCode::InvalidNonce)?;
        if &signer != ctx.accounts.contract_signer.to_account_info().key {
            return Err(ErrorCode::InvalidSigner.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut, has_one = vault)]
    content: ProgramAccount<'info, ContentRecord>,
    #[account(mut)]
    vault: CpiAccount<'info, TokenAccount>,
    #[account(seeds = [
        content.to_account_info().key.as_ref(),
        &[content.nonce],
    ])]
    contract_signer: AccountInfo<'info>,
    #[account(mut, has_one = owner)]
    owner_token: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    new_owner_token: CpiAccount<'info, TokenAccount>,
    #[account(signer)]
    new_owner: AccountInfo<'info>,
    owner: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}

fn check_funds(check: &ContentRecord, new_price: u64) -> Result<()> {
    if check.price >= new_price {
        return Err(ErrorCode::InsufficientFunds.into());
    }

    Ok(())
}

#[error]
pub enum ErrorCode {
    #[msg("The given nonce does not create a valid program derived address.")]
    InvalidNonce,
    #[msg("The derived signer does not match that which was given.")]
    InvalidSigner,
    #[msg("Insufficient funds provided to purchase route.")]
    InsufficientFunds,
}
