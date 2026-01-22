use anchor_lang::prelude::*;
use crate::{Intent, IntentStatus, transfer_tokens};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::error::IntentError;

#[derive(Accounts)]
pub struct CancelIntent<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = intent_account.creator == signer.key()
            @ IntentError::IncorrectOwner
    )]
    pub intent_account: Account<'info, Intent>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub owner_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = intent_account,
        associated_token::mint = token_a_mint,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn cancel_intent(ctx: Context<CancelIntent>) -> Result<()> {
    require!(
        ctx.accounts.intent_account.status == IntentStatus::Open,
        crate::error::IntentError::InvalidStatus
    );

    transfer_tokens(
        &ctx.accounts.vault,
        &ctx.accounts.owner_token_a_account,
        ctx.accounts.intent_account.token_a_amt,
        &ctx.accounts.token_a_mint,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    );

    ctx.accounts.intent_account.status = IntentStatus::Cancelled;
    Ok(())
}
