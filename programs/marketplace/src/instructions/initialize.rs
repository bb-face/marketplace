use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::marketplace::Marketplace;

#[derive(Accounts)]
// In the lesson 1:11:43
// Remove instruction name data and change `name.as_str()` for the marketplace seeds
// with "marketplace.name.as_str()" but doesn't compile for me ?
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
		init,
		payer = admin,
		seeds = [b"marketplace", name.as_str().as_bytes()],
		bump,
		space = Marketplace::INIT_SPACE,
	)]
    pub marketplace: Account<'info, Marketplace>,

    // Doesn't need to be initialized, it's a system account
    #[account(
			seeds = [b"treasury", marketplace.key().as_ref()],
			bump
		)]
    pub treasury: SystemAccount<'info>,

    // This is going to be responsible for minting the rewards to the
    // users account;
    #[account(
			init,
			payer = admin,
			seeds = [b"rewards", marketplace.key().as_ref()],
			bump,
			mint::decimals = 6,
			mint::authority = marketplace
		)]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.reward_mint,
            name,
        });

        Ok(())
    }
}
