use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FinalizeMintData {
    pub mint: Pubkey,
    pub weight: u64,
    pub bump: u8,
}

impl FinalizeMintData {
    pub fn init(&mut self, mint: Pubkey, weight: u64, bump: u8) -> Result<()> {
        self.mint = mint;
        self.weight = weight;
        self.bump = bump;

        Ok(())
    }
}
