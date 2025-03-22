use anchor_lang::prelude::*;

use crate::{
    constants::{FEES_COLLECTOR_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{fees_collector::FeesCollector, nft_manager::NFTManager},
};

#[derive(Accounts)]
pub struct UpdateFees<'info> {
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        address = nft_manager.authority @NFTManagerError::OnlyAdminAllowed
    )]
    pub signer: Signer<'info>,
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

impl<'info> UpdateFees<'info> {
    pub fn update_fees(&mut self, args: UpdateFeesArgs) -> Result<()> {
        match args.fee {
            Fees::FractionalizeFee => {
                self.fees_collector.fractionalize_fee = args.new_fee;
            }
            Fees::SellFee => {
                self.fees_collector.sell_fee = args.new_fee;
            }
        }

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateFeesArgs {
    pub fee: Fees,
    pub new_fee: u32,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum Fees {
    FractionalizeFee,
    SellFee,
}
