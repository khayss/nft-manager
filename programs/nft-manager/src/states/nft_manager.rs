use anchor_lang::prelude::*;

use crate::errors::NFTManagerError;

#[account]
#[derive(InitSpace)]
pub struct NFTManager {
    pub authority: Pubkey,
    pub future_authority: Option<Pubkey>,
    pub collection: Pubkey,
    pub bump: u8,
    pub discriminant: u64,
}

impl NFTManager {
    pub fn init(&mut self, authority: Pubkey, bump: u8, mint: Pubkey) -> Result<()> {
        self.authority = authority;
        self.bump = bump;
        self.collection = mint;
        self.discriminant = 0;
        self.future_authority = None;

        Ok(())
    }

    pub fn set_authority(&mut self, new_authority: Pubkey) -> Result<()> {
        require!(
            self.authority != new_authority,
            NFTManagerError::SameAuthority
        );
        self.authority = new_authority;
        self.future_authority = None;

        Ok(())
    }

    pub fn set_future_authority(&mut self, future_authority: Option<Pubkey>) -> Result<()> {
        self.future_authority = future_authority;

        Ok(())
    }

    pub fn increment_discriminant(&mut self) -> Result<()> {
        self.discriminant = self
            .discriminant
            .checked_add(1)
            .ok_or(NFTManagerError::Overflow)?;

        Ok(())
    }
}
