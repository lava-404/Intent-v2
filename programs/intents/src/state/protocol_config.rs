use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
  pub admin: Pubkey,
  pub max_intent_duration: i64,
  pub paused: bool,
}
