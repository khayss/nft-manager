use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface};

use crate::{
    constants::{LISTING_TAG, MINT_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::{listing::Listing, nft_manager::NFTManager},
};

#[derive(Accounts)]
#[instruction(args: UpdateListingPriceArgs)]
pub struct UpdateListingPrice<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,

    #[account(
        mut,
        address = listing.owner @NFTManagerError::NotOwner,
    )]
    pub owner: Signer<'info>,

    #[account(
        seeds = [MINT_TAG, &args.discriminant.to_le_bytes()],
        bump,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        mut,
        seeds = [LISTING_TAG, mint.key().as_ref()],
        bump,
        has_one = mint @NFTManagerError::InvalidListing,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> UpdateListingPrice<'info> {
    pub fn update_listing_price(&mut self, args: UpdateListingPriceArgs) -> Result<()> {
        self.listing.as_mut().price = args.new_price;

        emit!(UpdateListingPriceEvent {
            listing: self.listing.key(),
            new_price: args.new_price,
        });

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateListingPriceArgs {
    pub new_price: u64,
    pub discriminant: u64,
}
#[event]
pub struct UpdateListingPriceEvent {
    pub listing: Pubkey,
    pub new_price: u64,
}
