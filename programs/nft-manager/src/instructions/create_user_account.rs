use anchor_lang::prelude::*;

use crate::{
    constants::{NFT_MANAGER_TAG, USER_TAG},
    states::{nft_manager::NFTManager, user::User},
};

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + User::INIT_SPACE,
        seeds = [USER_TAG, owner.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> CreateUserAccount<'info> {
    pub fn create_user_account(&mut self, bumps: &CreateUserAccountBumps) -> Result<()> {
        let owner = self.owner.key();

        self.user_account.init(owner, bumps.user_account)?;

        emit!(CreateUserAccountEvent {
            user_account: self.user_account.key(),
            owner: self.owner.key(),
        });

        Ok(())
    }
}

#[event]
pub struct CreateUserAccountEvent {
    pub user_account: Pubkey,
    pub owner: Pubkey,
}
