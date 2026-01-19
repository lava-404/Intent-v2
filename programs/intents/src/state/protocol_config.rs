use anchor_lang::{AnchorDeserialize, AnchorSerialize, prelude::Pubkey};

#[derive(AnchorSerialize,
  AnchorDeserialize,)]
pub struct ProtocolConfig {
  pub admin: Pubkey,
  pub max_intent_duration: i64,
  pub paused: bool,
}
