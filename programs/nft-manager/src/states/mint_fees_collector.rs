use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MintFeesCollector {
    pub bump: u8,
}

impl MintFeesCollector {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;

        Ok(())
    }
}
