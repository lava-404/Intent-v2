use anchor_lang::prelude::*;

use crate::ProtocolConfig;

#[derive(Accounts)]

pub struct InitalizeConfig<'info> {
  #[account(mut)]
  pub initializer: Signer<'info>,

  #[account()]
  pub protocol_config: Account<'info, ProtocolConfig>
}

pub fn initalize_config(ctx: Context<InitalizeConfig>) {
  let protocol_config = ctx.accounts.protocol_config;
  protocol_config.admin = ctx.accounts.initializer.key();
  protocol_config.max_intent_duration = 3600;
  protocol_config.paused = false;
}