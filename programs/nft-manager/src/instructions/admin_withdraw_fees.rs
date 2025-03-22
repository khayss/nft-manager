use anchor_lang::prelude::*;

use crate::{
    constants::{FEES_COLLECTOR_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{fees_collector::FeesCollector, nft_manager::NFTManager},
};

#[derive(Accounts)]
pub struct AdminWithdrawFees<'info> {
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        address = nft_manager.authority @NFTManagerError::OnlyAdminAllowed,
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    /// CHECK: The account that will receive the withdrawn funds.
    pub recipient: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [FEES_COLLECTOR_TAG],
        bump,
    )]
    pub fees_collector: Account<'info, FeesCollector>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> AdminWithdrawFees<'info> {
    pub fn withdraw_fees(&mut self, amount: u64) -> Result<()> {
        // Ensure sufficient funds are available
        let current_balance = self.fees_collector.get_lamports();

        let rent_exempt_balance =
            Rent::get()?.minimum_balance(self.fees_collector.to_account_info().data_len());

        let withdrawable = current_balance - rent_exempt_balance;

        require_gt!(withdrawable, amount, NFTManagerError::InsufficientFunds);

        // Perform withdrawal
        self.fees_collector.sub_lamports(amount)?;
        self.recipient.add_lamports(amount)?;

        Ok(())
    }
}
