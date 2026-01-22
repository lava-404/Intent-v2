use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Intent, Solver};

#[derive(Accounts)]
pub struct SettleIntent<'info> {
  #[account(
    mut,
    constraint = solver_account.key() == solver.key()
  )]
  pub solver: Signer<'info>,
  #[account(
    mut,
)]
pub intent_account: Account<'info, Intent>,

#[account()]
pub solver_account: Account<'info, Solver>,

  #[account(mint::token_program = token_program)]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    payer = solver,
    associated_token::authority = intent_account,
    associated_token::mint = token_b_mint,
    associated_token::token_program = token_program
  )]
  pub output_vault: InterfaceAccount<'info, TokenAccount>,
  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>,
  pub associated_token_program: Program<'info, AssociatedToken>,
}

