use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

declare_id!("GvoS9ENU6jqPq1Put75pci4iEBxe5J96Z4Y2Frk26KP");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name:String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(
            name,
            fee,
            &ctx.bumps,
        )
    }
}

#[derive(Accounts)]
pub struct Initialize {}
