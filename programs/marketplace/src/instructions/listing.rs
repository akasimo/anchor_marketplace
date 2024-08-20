use anchor_lang::prelude::*;
use anchor_spl::{
    token::{spl_token::instruction::transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{state::Marketplace, Listing};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        seeds = [ b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        seeds = [ b"rewards", marketplace.key().as_ref()],
        bump = rewards_mint.bump,
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,

    maker_mint: Box<InterfaceAccount<'info, Mint>>,
    collection_mint: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    listing: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = listing,
        associated_token::mint = maker_mint
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [ b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,
    system_program: Program<'info>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> List<'info> {
    pub fn create_listing(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            mint: self.maker_mint.key(),
            price,
            bump: bumps.listing,
        });

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        let account: TransferChecked = TransferChecked {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), account);

        transfer_checked(self.t, source_pubkey, mint_pubkey, destination_pubkey, authority_pubkey, signer_pubkeys, amount, decimals)
        Ok(())
    }
}
