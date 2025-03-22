use anchor_lang::prelude::*;

use crate::{
    constants::{NFT_MANAGER_TAG, USER_TAG},
    errors::NFTManagerError,
    states::{nft_manager::NFTManager, user::User},
};

#[derive(Accounts)]
pub struct UserWithdraw<'info> {
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        address = user_account.authority @NFTManagerError::NotOwner,
    )]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> UserWithdraw<'info> {
    pub fn user_withdraw(&mut self, amount: u64) -> Result<()> {
        let bal = self.user_account.get_lamports();

        let rent_exempt_balance =
            Rent::get()?.minimum_balance(self.user_account.to_account_info().data_len());

        let withdrawable = bal - rent_exempt_balance;

        require_gte!(withdrawable, amount, NFTManagerError::InsufficientFunds);

        self.user_account.sub_lamports(amount)?;
        self.user.add_lamports(amount)?;

        emit!(UserWithdrawEvent {
            user: self.user.key(),
            amount,
        });

        Ok(())
    }
}

#[event]
pub struct UserWithdrawEvent {
    pub user: Pubkey,
    pub amount: u64,
}
