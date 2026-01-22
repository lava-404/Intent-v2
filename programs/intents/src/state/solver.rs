use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Solver {
  pub solver: Pubkey,
  pub stake: u64,
  pub active: bool
}