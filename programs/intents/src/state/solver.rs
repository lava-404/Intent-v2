use anchor_lang::prelude::Pubkey;

pub struct Solver {
  pub solver: Pubkey,
  pub stake: u64,
  pub active: bool
}