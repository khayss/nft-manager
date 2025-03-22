use anchor_lang::{prelude::*, solana_program::program::invoke_signed, system_program as system};
use anchor_spl::{token_2022::Token2022, token_interface};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{
        COLLECTION_KEY, COLLECTION_TAG, FEES_COLLECTOR_TAG, FINALIZE_FRACTIONALIZE_DATA_TAG,
        MINT_TAG, NFT_MANAGER_TAG, WEIGHT_KEY,
    },
    errors::NFTManagerError,
    states::{
        fees_collector::FeesCollector, finalize_fractionalize_data::FinalizeFractionalizeData,
        nft_manager::NFTManager,
    },
    utils,
};

use super::MintNFTArgs;

#[derive(Accounts)]
#[instruction(args: FractionalizeNFTArgs)]
pub struct FractionalizeNFT<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub gold_price_update: Box<Account<'info, PriceUpdateV2>>,
    pub sol_price_update: Box<Account<'info, PriceUpdateV2>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &args.discriminant.to_le_bytes()],
        bump,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
        constraint = token_account.amount == mint.supply @NFTManagerError::InvalidTokenAccount,
    )]
    pub token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        seeds = [COLLECTION_TAG],
        bump,
    )]
    /// CHECK: This account is validated
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + FinalizeFractionalizeData::calculate_space(args.part_b),
        seeds = [FINALIZE_FRACTIONALIZE_DATA_TAG, mint.key().as_ref(),],
        bump,
    )]
    pub finalize_data: Box<Account<'info, FinalizeFractionalizeData>>,

    #[account(
        mut,
        seeds = [FEES_COLLECTOR_TAG],
        bump,
    )]
    pub fees_collector: Box<Account<'info, FeesCollector>>,
    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> FractionalizeNFT<'info> {
    pub fn fractionalize_nft(
        &mut self,
        bumps: &FractionalizeNFTBumps,
        args: FractionalizeNFTArgs,
    ) -> Result<()> {
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();

        let FractionalizeNFTArgs {
            discriminant,
            part_b,
            part_a,
        } = args;

        let token_metadata = utils::get_metadata(mint)?;

        let weight = utils::get_weight(&token_metadata.additional_metadata)?;
        utils::validate_fractions(weight, part_a.weight, part_b.weight)?;

        let value_in_lamports = utils::get_gold_value_in_lamport(
            &self.gold_price_update,
            &self.sol_price_update,
            weight,
        )?;

        let fees = value_in_lamports
            .checked_mul(self.fees_collector.fractionalize_fee as u64)
            .and_then(|v| v.checked_div(10u64.pow(self.fees_collector.fees_decimals as u32)))
            .ok_or(NFTManagerError::Overflow)?;

        system::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                system::Transfer {
                    from: self.signer.to_account_info(),
                    to: self.fees_collector.to_account_info(),
                },
            ),
            fees,
        )?;

        let mint_seeds = &[MINT_TAG, &discriminant.to_le_bytes(), &[bumps.mint]];
        let mint_signer_seeds = &[&mint_seeds[..]];

        invoke_signed(
            &token_interface::spl_token_metadata_interface::instruction::remove_key(
                &token_program.key(),   // token program id
                &mint.key(),            // "metadata" account
                &mint.key(),            // update authority
                WEIGHT_KEY.to_string(), // key to remove
                true, // idempotent flag, if true transaction will not fail if key does not exist
            ),
            &[token_program.clone(), mint.clone(), mint.clone()],
            mint_signer_seeds,
        )?;

        invoke_signed(
            &token_interface::spl_token_metadata_interface::instruction::remove_key(
                &token_program.key(),       // token program id
                &mint.key(),                // "metadata" account
                &mint.key(),                // update authority
                COLLECTION_KEY.to_string(), // key to remove
                true, // idempotent flag, if true transaction will not fail if key does not exist
            ),
            &[token_program.clone(), mint.clone(), mint.clone()],
            mint_signer_seeds,
        )?;

        utils::update_metadata(
            token_program.clone(),
            mint.clone(),
            mint_signer_seeds,
            part_a.weight,
            &part_a.name,
            &part_a.symbol,
            &part_a.uri,
            &self.collection.key().to_string(),
        )?;

        self.finalize_data
            .as_mut()
            .init(mint.key(), part_b, bumps.finalize_data)?;

        emit!(FractionalizeNFTEvent {
            mint: mint.key(),
            finalize_data: self.finalize_data.key(),
            discriminant,
        });

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FractionalizeNFTArgs {
    pub discriminant: u64,
    pub part_b: MintNFTArgs,
    pub part_a: MintNFTArgs,
}

#[event]
pub struct FractionalizeNFTEvent {
    pub mint: Pubkey,
    pub finalize_data: Pubkey,
    pub discriminant: u64,
}
