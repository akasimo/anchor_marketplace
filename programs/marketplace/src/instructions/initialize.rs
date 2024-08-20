use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint};

use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        init, 
        payer = admin, 
        space = Marketplace::INIT_SPACE,
        seeds = [ b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init, 
        payer = admin,
        seeds = [ b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = marketplace,
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [ b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,
    system_program: AccountInfo<'info>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        name: String,
        fee: u16,
        bumps: &InitializeBumps
    ) -> Result<()> {
        // require!(
        //     name.len() <= 32 && name.len() > 0,
        //     "name must be less than or equal to 32 characters"
        // )

        self.marketplace.set_inner(Marketplace {
            authority: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            rewards_bump: bumps.rewards,
            treasury_bump: bumps.treasury,
            name,
        });

        Ok(())
    }
}