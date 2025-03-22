use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::Token2022, token_interface};

use crate::{
    constants::{LISTING_TAG, LISTING_TOKEN_ACCOUNT_TAG, MINT_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{listing::Listing, nft_manager::NFTManager},
};

#[derive(Accounts)]
#[instruction(discriminant: u64)]
pub struct DelistNFT<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(
        mut,
        address = listing.owner @NFTManagerError::NotOwner,
    )]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &discriminant.to_le_bytes()],
        bump,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_TAG, mint.key().as_ref(),],
        bump,
        close = owner,
        has_one = mint @NFTManagerError::InvalidListing,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = listing_token_account,
        seeds = [LISTING_TOKEN_ACCOUNT_TAG, listing.key().as_ref()],
        bump
    )]
    pub listing_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> DelistNFT<'info> {
    pub fn delist_nft(&mut self, bumps: &DelistNFTBumps) -> Result<()> {
        let mint = self.mint.to_account_info();
        let listing_token_account = self.listing_token_account.to_account_info();
        let owner = self.owner.to_account_info();
        let listing_key = self.listing.key();

        let listing_token_account_seeds = &[
            LISTING_TOKEN_ACCOUNT_TAG,
            listing_key.as_ref(),
            &[bumps.listing_token_account],
        ];
        let listing_token_account_signer_seeds = &[&listing_token_account_seeds[..]];

        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token_interface::TransferChecked {
                    from: listing_token_account.clone(),
                    mint: mint.clone(),
                    to: self.owner_token_account.to_account_info(),
                    authority: listing_token_account,
                },
                listing_token_account_signer_seeds,
            ),
            1,
            0,
        )?;

        emit!(DelistNFTEvent {
            owner: owner.key(),
            mint: mint.key(),
        });

        Ok(())
    }
}

#[event]
pub struct DelistNFTEvent {
    pub owner: Pubkey,
    pub mint: Pubkey,
}
