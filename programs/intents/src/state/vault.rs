use anchor_lang::prelude::Pubkey;

pub struct Vault {
  pub intent: Pubkey,
  pub token_stored_mnt: Pubkey,
  pub amt_stored: u64,
  pub bump: u8
}