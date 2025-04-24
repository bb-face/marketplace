use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
		seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
		bump = marketplace.bump,
	)]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
		init_if_needed,
		payer = taker,
		associated_token::mint = maker_mint,
		associated_token::authority = taker,
	)]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
		init_if_needed,
		payer = taker,
		associated_token::mint = maker_mint,
		associated_token::authority = taker,
	)]
    pub taker_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
			mut,
		associated_token::mint = maker_mint,
		associated_token::authority = listing,
	)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
			mut,
			close = maker,
		seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
		bump = listing.bump,
	)]
    pub listing: Account<'info, Listing>,

    #[account(
			seeds = [b"treasury", marketplace.key().as_ref()],
			bump
		)]
    pub treasury: SystemAccount<'info>,

    #[account(
			mut,
			seeds = [b"rewards", marketplace.key().as_ref()],
			bump = marketplace.rewards_bump,
			mint::decimals = 6,
			mint::authority = marketplace
		)]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
		seeds = [
			b"metadata",
			metadata_program.key().as_ref(),
			maker_mint.key().as_ref(),
		],
		seeds::program = metadata_program.key(),
		bump,
		constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
		constraint = metadata.collection.as_ref().unwrap().verified == true,
	)]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
		seeds = [
			b"metadata",
			metadata_program.key().as_ref(),
			maker_mint.key().as_ref(),
			b"edition",
		],
		seeds::program = metadata_program.key(),
		bump
	)]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub metadata_program: Program<'info, Metadata>, // Metaplex program
    pub associated_token_program: Program<'info, AssociatedToken>, // For creating ATAs
    pub system_program: Program<'info, System>,     // For creating accounts
    pub token_program: Interface<'info, TokenInterface>, // For token operations
}
impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        let marketplace_fee = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price)
            .unwrap()
            .checked_div(10000_u64)
            .unwrap();

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
            // authority: self.taker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let amount = self.listing.price.checked_sub(marketplace_fee).unwrap();

        transfer(cpi_context, amount);

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        Ok(())
    }

    pub fn send_nft(&mut self) -> Result<()> {
        todo!()
    }
}
