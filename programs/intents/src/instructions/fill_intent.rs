use anchor_lang::prelude::*;

use crate::{Intent, IntentStatus, Solver};
#[derive(Accounts)]
#[instruction(stake: u64)]

pub struct FillIntent<'info> {
  #[account(mut)]
  pub solver: Signer<'info>,

  #[account(
    init,
    payer = solver,
    space = 8 + Solver::INIT_SPACE,
    seeds = [b"solver", stake.to_le_bytes().as_ref()],
    bump
  )]


  pub solver_account: Account<'info, Solver>,

  #[account(
    mut
  )]
  pub intent_account: Account<'info, Intent>,

  pub system_program: Program<'info, System>

}


pub fn fill_intent(ctx: Context<FillIntent>) -> Result<()> {
  require!(ctx.accounts.intent_account.status == IntentStatus::Open, IntentError::IntentNotOpen);
  require!(ctx.accounts.intent_account.expiry > Clock::get()?.unix_timestamp, IntentError::IntentExpired);

  ctx.accounts.intent_account.solver = ctx.accounts.solver.key();
  ctx.accounts.intent_account.status = IntentStatus::Filled;

  Ok(())
  
}
