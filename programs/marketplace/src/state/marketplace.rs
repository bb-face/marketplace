use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    // Store the fees in the treasury, that's is a PDA
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 3 * 1 + (4 + 32); // 32 is a user made assumption,
                                                             // check the doc: https://www.anchor-lang.com/docs/references/space
}
