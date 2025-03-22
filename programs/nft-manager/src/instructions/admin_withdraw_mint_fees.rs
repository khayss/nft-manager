use anchor_lang::prelude::*;

use crate::{
    constants::{MINT_FEES_COLLECTOR_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{mint_fees_collector::MintFeesCollector, nft_manager::NFTManager},
};

#[derive(Accounts)]
pub struct AdminWithdrawMintFees<'info> {
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
        seeds = [MINT_FEES_COLLECTOR_TAG],
        bump,
    )]
    pub mint_fees_collector: Account<'info, MintFeesCollector>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> AdminWithdrawMintFees<'info> {
    pub fn withdraw_mint_fees(&mut self, amount: u64) -> Result<()> {
        let current_balance = self.mint_fees_collector.get_lamports();

        let rent_exempt_balance =
            Rent::get()?.minimum_balance(self.mint_fees_collector.to_account_info().data_len());

        let withdrawable = current_balance - rent_exempt_balance;

        require_gt!(withdrawable, amount, NFTManagerError::InsufficientFunds);

        // Perform withdrawal
        self.mint_fees_collector.sub_lamports(amount)?;
        self.recipient.add_lamports(amount)?;

        Ok(())
    }
}
