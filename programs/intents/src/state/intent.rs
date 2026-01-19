use anchor_lang::prelude::*;
#[derive(
  AnchorSerialize,
  AnchorDeserialize,
  Clone,
  Copy,
  InitSpace,
  PartialEq,
  Eq
)]
pub enum IntentStatus {
  Open,
  Filled,
  Settled, 
  Cancelled,
  Expired
}

#[derive(InitSpace, AnchorSerialize,
  AnchorDeserialize,)]
pub struct Intent {
  pub creator: Pubkey,
  pub token_a_mint: Pubkey,
  pub token_a_amt: u64,
  pub token_b_mint: Pubkey,
  pub token_b_amt_min: u64,
  pub recipient:  Pubkey,
  pub created_at: i64,
  pub expiry: i64,
  pub solver: Option<Pubkey>,
  pub bump: u8,
  pub status: IntentStatus
}

