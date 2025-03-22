use anchor_lang::prelude::*;

use crate::{
    constants::{COLLECTION_TAG, FEES_COLLECTOR_TAG, MINT_FEES_COLLECTOR_TAG, NFT_MANAGER_TAG},
    states::{
        fees_collector::FeesCollector, mint_fees_collector::MintFeesCollector,
        nft_manager::NFTManager,
    },
};

#[derive(Accounts)]

pub struct InitializeNFTManager<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [COLLECTION_TAG],
        bump,

    )]
    /// CHECK: This account is validated via the seeds
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + FeesCollector::INIT_SPACE,
        seeds = [FEES_COLLECTOR_TAG],
        bump
    )]
    pub fees_collector: Account<'info, FeesCollector>,

    #[account(
        init,
        payer = signer,
        space = 8 + MintFeesCollector::INIT_SPACE,
        seeds = [MINT_FEES_COLLECTOR_TAG],
        bump
    )]
    pub mint_fees_collector: Account<'info, MintFeesCollector>,

    #[account(
        init,
        payer = signer,
        space = 8 + NFTManager::INIT_SPACE,
        seeds = [NFT_MANAGER_TAG],
        bump
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> InitializeNFTManager<'info> {
    pub fn initialize_nft_manager(
        &mut self,
        bumps: &InitializeNFTManagerBumps,
        args: InitializeNFTManagerArgs,
    ) -> Result<()> {
        self.nft_manager
            .init(self.signer.key(), bumps.nft_manager, self.collection.key())?;

        self.fees_collector
            .init(args.fractionalize_fee, args.sell_fee, bumps.fees_collector)?;

        self.mint_fees_collector.init(bumps.mint_fees_collector)?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeNFTManagerArgs {
    pub fractionalize_fee: u32,
    pub sell_fee: u32,
}
