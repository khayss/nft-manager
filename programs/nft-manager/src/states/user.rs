use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub authority: Pubkey,
    pub bump: u8,
}

impl User {
    pub fn init(&mut self, authority: Pubkey, bump: u8) -> Result<()> {
        self.authority = authority;
        self.bump = bump;

        Ok(())
    }
}
