use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface};

use crate::{
    constants::{COLLECTION_TAG, FINALIZE_MINT_DATA_TAG, MINT_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{finalize_mint_data::FinalizeMintData, nft_manager::NFTManager},
    utils,
};

#[derive(Accounts)]
#[instruction(discriminant: u64)]
pub struct FinalizeMintNFT<'info> {
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &discriminant.to_le_bytes()],
        bump,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        seeds = [COLLECTION_TAG],
        bump,
    )]
    /// CHECK: This account is validated
    pub collection: UncheckedAccount<'info>,

    #[account(
        mut,
        close = signer,
        seeds = [FINALIZE_MINT_DATA_TAG, mint.key().as_ref()],
        bump,
        has_one = mint @NFTManagerError::InvalidFinalizeData,
    )]
    pub finalize_data: Box<Account<'info, FinalizeMintData>>,

    #[account(
        mut,
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> FinalizeMintNFT<'info> {
    pub fn finalize_mint_nft(
        &mut self,
        bumps: &FinalizeMintNFTBumps,
        discriminant: u64,
    ) -> Result<()> {
        let collection_key = self.collection.key();
        let weight = self.finalize_data.weight;
        let mint = self.mint.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let payer = self.signer.to_account_info();

        let mint_seeds = &[MINT_TAG, &discriminant.to_le_bytes()[..], &[bumps.mint]];
        let mint_signer_seeds = &[&mint_seeds[..]];

        utils::update_metadata_standard(
            mint,
            system_program,
            token_program,
            payer,
            collection_key,
            weight,
            mint_signer_seeds,
        )?;

        emit!(FinalizeMintNFTEvent {
            mint: self.mint.key(),
            weight,
        });

        Ok(())
    }
}

#[event]
pub struct FinalizeMintNFTEvent {
    pub mint: Pubkey,
    pub weight: u64,
}
