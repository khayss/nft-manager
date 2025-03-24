use anchor_lang::{prelude::*, system_program as system};
use anchor_spl::{associated_token::AssociatedToken, token_2022::Token2022, token_interface};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{FINALIZE_MINT_DATA_TAG, MINT_FEES_COLLECTOR_TAG, MINT_TAG, NFT_MANAGER_TAG},
    states::{
        finalize_mint_data::FinalizeMintData, mint_fees_collector::MintFeesCollector,
        nft_manager::NFTManager,
    },
    utils,
};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    pub gold_price_update: Box<Account<'info, PriceUpdateV2>>,
    pub sol_price_update: Box<Account<'info, PriceUpdateV2>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [MINT_TAG, &nft_manager.discriminant.to_le_bytes()],
        bump,
        mint::decimals = 0,
        mint::authority = mint,
        mint::freeze_authority = mint,
        extensions::metadata_pointer::metadata_address = mint,
        extensions::metadata_pointer::authority = mint,
        extensions::group_member_pointer::authority = mint,
        extensions::group_member_pointer::member_address = mint,
        extensions::close_authority::authority = mint,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    /// CHECK: This account will receive the token
    pub recipient: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = recipient,
        associated_token::token_program = token_program
    )]
    pub recipient_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        init,
        payer = signer,
        space = 8 + FinalizeMintData::INIT_SPACE,
        seeds = [FINALIZE_MINT_DATA_TAG, mint.key().as_ref()],
        bump,
    )]
    pub finalize_data: Box<Account<'info, FinalizeMintData>>,

    #[account(
        mut,
        seeds = [MINT_FEES_COLLECTOR_TAG],
        bump,
    )]
    pub mint_fees_collector: Box<Account<'info, MintFeesCollector>>,

    #[account(
        mut,
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(&mut self, bumps: &MintNFTBumps, args: MintNFTArgs) -> Result<()> {
        let mint_key = self.mint.key();
        let signer = self.signer.to_account_info();
        let system_program = self.system_program.to_account_info();
        let finalize_data = self.finalize_data.as_mut();
        let discriminant = self.nft_manager.discriminant;

        let price_in_lamports = utils::get_gold_value_in_lamport(
            &self.gold_price_update,
            &self.sol_price_update,
            args.weight,
        )?;

        system::transfer(
            CpiContext::new(
                system_program.clone(),
                system::Transfer {
                    from: signer.clone(),
                    to: self.mint_fees_collector.to_account_info(),
                },
            ),
            price_in_lamports,
        )?;

        let mint_seeds = &[MINT_TAG, &discriminant.to_le_bytes()[..], &[bumps.mint]];
        let mint_signer_seeds = &[&mint_seeds[..]];

        utils::mint_nft(
            args.name,
            args.symbol,
            args.uri,
            args.weight,
            discriminant,
            mint_signer_seeds,
            &mut self.nft_manager,
            self.mint.to_account_info(),
            self.recipient_token_account.to_account_info(),
            self.token_program.to_account_info(),
            system_program,
            signer,
        )?;

        finalize_data.init(mint_key, args.weight, bumps.finalize_data)?;

        emit!(MintNFTEvent {
            mint: mint_key,
            finalize_data: finalize_data.key(),
            recipient: self.recipient.key(),
            price: price_in_lamports,
            discriminant,
        });

        Ok(())
    }
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct MintNFTArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub weight: u64,
}

#[event]
pub struct MintNFTEvent {
    pub mint: Pubkey,
    pub finalize_data: Pubkey,
    pub recipient: Pubkey,
    pub price: u64,
    pub discriminant: u64,
}
