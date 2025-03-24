use anchor_lang::{
    prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program as system,
};
use anchor_spl::{associated_token::AssociatedToken, token_2022::Token2022, token_interface};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    constants::{
        FEES_COLLECTOR_TAG, LISTING_TAG, LISTING_TOKEN_ACCOUNT_TAG, LIST_NFT_PRICE_DECIMALS,
        MAX_AGE, MINT_TAG, NFT_MANAGER_TAG, SOL_PRICE_FEED_ID_HEX, USER_TAG,
    },
    errors::NFTManagerError,
    states::{
        fees_collector::FeesCollector, listing::Listing, nft_manager::NFTManager, user::User,
    },
};

#[derive(Accounts)]
#[instruction(discriminant: u64)]
pub struct BuyNFT<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub sol_price_update: Box<Account<'info, PriceUpdateV2>>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        address = listing.owner @NFTManagerError::NotOwner,
    )]
    /// CHECK: The owner of the listing
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &discriminant.to_le_bytes()],
        bump,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        mut,
        close = seller_account,
        seeds = [LISTING_TAG, mint.key().as_ref(), seller.key().as_ref()],
        bump,
        has_one = mint @NFTManagerError::InvalidListing,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        seeds = [LISTING_TOKEN_ACCOUNT_TAG, listing.key().as_ref()],
        bump,
    )]
    pub listing_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        mut,
        seeds = [USER_TAG, seller.key().as_ref()],
        bump,
        constraint = seller_account.authority == seller.key() @NFTManagerError::UnAuthorized,
    )]
    pub seller_account: Box<Account<'info, User>>,

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

impl<'info> BuyNFT<'info> {
    pub fn buy_nft(&mut self, bumps: &BuyNFTBumps) -> Result<()> {
        let buyer = self.buyer.to_account_info();
        let mint = self.mint.to_account_info();
        let system_program = self.system_program.to_account_info();
        let seller = self.seller.to_account_info();
        let listing = self.listing.to_account_info();
        let listing_token_account = self.listing_token_account.to_account_info();

        let price = self.listing.price as u128;
        let fees_percentage = self.fees_collector.sell_fee;
        let fees_decimals = self.fees_collector.fees_decimals;

        let sol_feed_id: [u8; 32] = get_feed_id_from_hex(SOL_PRICE_FEED_ID_HEX)?;
        let sol_price =
            self.sol_price_update
                .get_price_no_older_than(&Clock::get()?, MAX_AGE, &sol_feed_id)?;

        require_gt!(sol_price.price, 0, NFTManagerError::NegativePrice);

        let mut numerator = price
            .checked_mul(LAMPORTS_PER_SOL as u128)
            .ok_or(NFTManagerError::Overflow)?;
        let mut denominator = (sol_price.price as u128)
            .checked_sub(sol_price.conf as u128)
            .ok_or(NFTManagerError::Overflow)?;

        let exponent_diff = -(LIST_NFT_PRICE_DECIMALS as i32) - sol_price.exponent;

        if exponent_diff < 0 {
            denominator = denominator
                .checked_mul(10u128.pow(exponent_diff as u32))
                .ok_or(NFTManagerError::Overflow)?;
        } else {
            numerator = numerator
                .checked_mul(10u128.pow(exponent_diff as u32))
                .ok_or(NFTManagerError::Overflow)?;
        }

        let price_in_lamports: u64 = numerator
            .checked_div(denominator)
            .ok_or(NFTManagerError::Overflow)?
            .try_into()
            .map_err(|_| NFTManagerError::Overflow)?;

        let fees = price_in_lamports
            .checked_mul(fees_percentage as u64)
            .ok_or(NFTManagerError::Overflow)?
            .checked_div(10u64.pow(fees_decimals as u32))
            .ok_or(NFTManagerError::Overflow)?;

        system::transfer(
            CpiContext::new(
                system_program.clone(),
                system::Transfer {
                    from: buyer.clone(),
                    to: self.fees_collector.to_account_info(),
                },
            ),
            fees,
        )?;

        system::transfer(
            CpiContext::new(
                system_program,
                system::Transfer {
                    from: buyer.clone(),
                    to: self.seller_account.to_account_info(),
                },
            ),
            price_in_lamports,
        )?;

        let listing_key = listing.key();
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
                    mint: mint.clone(),
                    authority: listing_token_account.clone(),
                    from: listing_token_account,
                    to: self.buyer_token_account.to_account_info(),
                },
                listing_token_account_signer_seeds,
            ),
            1,
            0,
        )?;

        emit!(BuyNFTEvent {
            buyer: buyer.key(),
            seller: seller.key(),
            mint: mint.key(),
            price: self.listing.price,
        });

        Ok(())
    }
}

#[event]
pub struct BuyNFTEvent {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
}
