use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::Token2022, token_interface};

use crate::{
    constants::{COLLECTION_TAG, FINALIZE_FRACTIONALIZE_DATA_TAG, MINT_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{finalize_fractionalize_data::FinalizeFractionalizeData, nft_manager::NFTManager},
    utils,
};

#[derive(Accounts)]
#[instruction(discriminant: u64)]
pub struct FinalizeFractionalizeNFT<'info> {
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &discriminant.to_le_bytes()],
        bump,
        address = finalize_data.mint @NFTManagerError::MintFinalizeDataMismatch
    )]
    /// CHECK: The mint account to finalize
    pub mint: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [MINT_TAG, &nft_manager.discriminant.to_le_bytes()],
        bump,
        mint::decimals = 0,
        mint::authority = new_mint,
        mint::freeze_authority = new_mint,
        extensions::metadata_pointer::metadata_address = new_mint,
        extensions::metadata_pointer::authority = new_mint,
        extensions::group_member_pointer::authority = new_mint,
        extensions::group_member_pointer::member_address = new_mint,
        extensions::close_authority::authority = new_mint,
    )]
    pub new_mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = new_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub new_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        seeds = [COLLECTION_TAG],
        bump,
    )]
    /// CHECK: This account is validated
    pub collection: UncheckedAccount<'info>,

    #[account(
        mut,
        close = signer,
        seeds = [FINALIZE_FRACTIONALIZE_DATA_TAG, mint.key().as_ref()],
        bump,
    )]
    pub finalize_data: Box<Account<'info, FinalizeFractionalizeData>>,

    #[account(
        mut,
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> FinalizeFractionalizeNFT<'info> {
    pub fn finalize_mint_nft(&mut self, bumps: &FinalizeFractionalizeNFTBumps) -> Result<()> {
        let collection_key = self.collection.key();
        let new_mint_discriminant = self.nft_manager.discriminant;
        let weight = self.finalize_data.weight;
        let new_mint = self.new_mint.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let payer = self.signer.to_account_info();

        let new_mint_seeds = &[
            MINT_TAG,
            &new_mint_discriminant.to_le_bytes(),
            &[bumps.new_mint],
        ];

        let new_mint_signer_seeds = &[&new_mint_seeds[..]];

        utils::mint_nft(
            self.finalize_data.name.clone(),
            self.finalize_data.symbol.clone(),
            self.finalize_data.uri.clone(),
            weight,
            new_mint_discriminant,
            new_mint_signer_seeds,
            &mut self.nft_manager,
            self.new_mint.to_account_info(),
            self.new_token_account.to_account_info(),
            token_program.clone(),
            self.system_program.to_account_info(),
            self.signer.to_account_info(),
        )?;

        utils::update_metadata_standard(
            new_mint,
            system_program,
            token_program,
            payer,
            collection_key,
            weight,
            new_mint_signer_seeds,
        )?;

        emit!(FinalizeFractionalizeNFTEvent {
            mint: self.mint.key(),
            new_mint: self.new_mint.key()
        });

        Ok(())
    }
}

#[event]
pub struct FinalizeFractionalizeNFTEvent {
    pub mint: Pubkey,
    pub new_mint: Pubkey,
}
