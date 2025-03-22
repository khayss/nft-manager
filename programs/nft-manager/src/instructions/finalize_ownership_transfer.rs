use anchor_lang::prelude::*;

use crate::{constants::NFT_MANAGER_TAG, errors::NFTManagerError, states::nft_manager::NFTManager};

#[derive(Accounts)]
pub struct FinalizeOwnershipTransfer<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> FinalizeOwnershipTransfer<'info> {
    pub fn finalize_ownership_transfer(&mut self) -> Result<()> {
        let future_owner = self.nft_manager.future_authority;
        let signer = self.signer.key();

        if let Some(some_future_owner) = future_owner {
            require_eq!(
                signer,
                some_future_owner,
                NFTManagerError::OnlyFutureAuthorityAllowed
            );

            self.nft_manager.set_authority(self.signer.key())?;
        } else {
            return Err(NFTManagerError::NoFutureAuthority.into());
        }

        Ok(())
    }
}
