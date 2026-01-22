use anchor_lang::prelude::*;
use crate::{Intent, IntentStatus, ProtocolConfig, transfer_tokens};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::error::IntentError;
#[derive(Accounts)]
#[instruction(created_at: i64)]
pub struct OpenIntent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        space = 8 + Intent::INIT_SPACE,
        seeds = [b"intent", owner.key().as_ref(), created_at.to_le_bytes().as_ref()],
        payer = owner,
        bump
    )]
    pub intent_account: Account<'info, Intent>,

    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        associated_token::authority = intent_account,
        associated_token::mint = token_a_mint,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn open_intent (
    ctx: Context<OpenIntent>,
    token_a_amt: u64,
    recipient: Pubkey,
    expiry: i64,
    token_b_amt_min: u64,
    token_b_mint: Pubkey,
) -> Result<()> {
    let intent_account = &mut ctx.accounts.intent_account;
    let created_at = Clock::get()?.unix_timestamp;

    require!(
        token_a_amt > 0,
        IntentError::InvalidAmount
    );  
    

    require!(
        token_b_amt_min > 0,
        IntentError::InvalidMinOutput
    );

    require!(
        expiry > created_at,
        IntentError::InvalidExpiry
    );

    let duration = expiry
        .checked_sub(created_at)
        .ok_or(IntentError::InvalidExpiry)?;

    require!(
        duration <= ctx.accounts.protocol_config.max_intent_duration,
        IntentError::InvalidExpiry
    );

    require!(
        !ctx.accounts.protocol_config.paused,
        IntentError::ProtocolPaused
    );

    require!(
        recipient != Pubkey::default(),
        IntentError::InvalidRecipient
    );

    intent_account.creator = ctx.accounts.owner.key();
    intent_account.token_a_mint = ctx.accounts.token_a_mint.key();
    intent_account.token_a_amt = token_a_amt;
    intent_account.token_b_mint = token_b_mint;
    intent_account.token_b_amt_min = token_b_amt_min;
    intent_account.recipient = recipient;
    intent_account.created_at = created_at;
    intent_account.expiry = expiry;
    intent_account.status = IntentStatus::Open;
    intent_account.bump = ctx.bumps.intent_account;

    transfer_tokens(
        &ctx.accounts.owner_token_a_account,
        &ctx.accounts.vault,
        token_a_amt,
        &ctx.accounts.token_a_mint,
        &ctx.accounts.owner,
        &ctx.accounts.token_program,
    );

  Ok(())
}
