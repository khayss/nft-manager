use anchor_lang::prelude::*;

use crate::{constants::NFT_MANAGER_TAG, errors::NFTManagerError, states::nft_manager::NFTManager};

#[derive(Accounts)]
pub struct InitializeOwnershipTransfer<'info> {
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        address = nft_manager.authority @NFTManagerError::OnlyAdminAllowed,
    )]
    pub signer: Signer<'info>,

    /// CHECK: This is the new owner of the NFT Manager
    pub new_owner: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> InitializeOwnershipTransfer<'info> {
    pub fn initialize_transfer_ownership(&mut self) -> Result<()> {
        let future_owner = self.new_owner.key();

        require_neq!(
            self.signer.key(),
            future_owner,
            NFTManagerError::SameAuthority
        );

        self.nft_manager.set_future_authority(Some(future_owner))?;

        Ok(())
    }
}
