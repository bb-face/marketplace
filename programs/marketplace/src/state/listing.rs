use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub maker: Pubkey,
    // Nft that's going to be listed by the maker
    pub maker_mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 2 * 32 + 8 + 1;
}
