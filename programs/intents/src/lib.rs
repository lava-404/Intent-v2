pub mod constants;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub mod error;
pub use error::*;

declare_id!("AbcKbAuU5ha9Kw2PywcSfGowXfWqwLcLxvb5sLrCyFUC");

#[program]
pub mod intents {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
