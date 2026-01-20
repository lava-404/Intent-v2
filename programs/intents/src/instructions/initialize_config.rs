use anchor_lang::prelude::*;

use crate::ProtocolConfig;

#[derive(Accounts)]

pub struct InitalizeConfig<'info> {
  #[account(mut)]
  pub initializer: Signer<'info>,

  #[account(
    init,
    payer = initializer,
    space = 8 + ProtocolConfig::INIT_SPACE,
    seeds = [b"config"],
    bump
  )]
  pub protocol_config: Account<'info, ProtocolConfig>,

  pub system_program: Program<'info, System>,

}

pub fn initalize_config(ctx: Context<InitalizeConfig>) -> Result<()> {
  let protocol_config = &mut ctx.accounts.protocol_config;
  protocol_config.admin = ctx.accounts.initializer.key();
  protocol_config.max_intent_duration = 3600;
  protocol_config.paused = false;

  Ok(())
}