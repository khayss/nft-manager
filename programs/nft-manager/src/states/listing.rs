use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub price: u64,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
}

impl Listing {
    pub fn init(&mut self, price: u64, owner: Pubkey, mint: Pubkey, bump: u8) -> Result<()> {
        self.owner = owner;
        self.price = price;
        self.mint = mint;
        self.bump = bump;

        Ok(())
    }

    pub fn update_price(&mut self, price: u64) -> Result<()> {
        self.price = price;

        Ok(())
    }
}
