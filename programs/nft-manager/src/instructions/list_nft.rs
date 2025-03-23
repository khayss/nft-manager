use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface};

use crate::{
    constants::{
        COLLECTION_TAG, LISTING_TAG, LISTING_TOKEN_ACCOUNT_TAG, MINT_TAG, NFT_MANAGER_TAG,
    },
    errors::NFTManagerError,
    states::{listing::Listing, nft_manager::NFTManager},
    utils,
};

#[derive(Accounts)]
#[instruction(discriminant: u64)]
pub struct ListNFT<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [MINT_TAG, &discriminant.to_le_bytes()],
        bump,
        constraint = mint.supply == 1 @NFTManagerError::InvalidMintSupply,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        seeds = [COLLECTION_TAG],
        bump,
    )]
    /// CHECK: This account is validated
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Listing::INIT_SPACE,
        seeds = [LISTING_TAG, mint.key().as_ref(),],
        bump
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        init_if_needed,
        payer = owner,
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

impl<'info> ListNFT<'info> {
    pub fn list_nft(&mut self, bumps: &ListNFTBumps, price: u64) -> Result<()> {
        let owner = self.owner.to_account_info();
        let owner_token_account = self.owner_token_account.to_account_info();
        let mint = self.mint.to_account_info();

        let token_metadata = utils::get_metadata(&mint)?;

        utils::validate_collection(&token_metadata.additional_metadata, self.collection.key())?;

        utils::validate_weight(&token_metadata.additional_metadata)?;

        self.listing
            .as_mut()
            .init(price, owner.key(), mint.key(), bumps.listing)?;

        token_interface::transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                token_interface::TransferChecked {
                    authority: owner.clone(),
                    from: owner_token_account,
                    mint: mint.clone(),
                    to: self.listing_token_account.to_account_info(),
                },
            ),
            1,
            0,
        )?;

        emit!(ListNFTEvent {
            owner: owner.key(),
            listing: self.listing.key(),
            mint: mint.key(),
            price,
        });

        Ok(())
    }
}

#[event]
pub struct ListNFTEvent {
    pub owner: Pubkey,
    pub listing: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
}
