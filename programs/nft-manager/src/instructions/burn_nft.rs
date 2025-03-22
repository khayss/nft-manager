use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface};

use crate::{
    constants::{MINT_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::nft_manager::NFTManager,
    utils,
};

#[derive(Accounts)]
#[instruction(discriminant: u64)]
pub struct BurnNFT<'info> {
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        address = nft_manager.authority @NFTManagerError::OnlyAdminAllowed
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &discriminant.to_le_bytes()],
        bump,
        constraint = mint.supply > 0 @NFTManagerError::InvalidMintSupply,
    )]
    pub mint: InterfaceAccount<'info, token_interface::Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
        constraint = token_account.amount == mint.supply @NFTManagerError::InvalidTokenAccount,
    )]
    pub token_account: InterfaceAccount<'info, token_interface::TokenAccount>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Box<Account<'info, NFTManager>>,
}

impl<'info> BurnNFT<'info> {
    pub fn burn_nft(&mut self, bumps: &BurnNFTBumps, discriminant: u64) -> Result<()> {
        let token_program = self.token_program.to_account_info();
        let mint = self.mint.to_account_info();
        let signer = self.signer.to_account_info();
        let source = self.token_account.to_account_info();
        let amount = self.mint.supply;

        let mint_seeds = &[MINT_TAG, &discriminant.to_le_bytes(), &[bumps.mint]];
        let mint_signer_seeds = &[&mint_seeds[..]];

        // Get and update the token metadata
        let token_metadata = utils::get_metadata(&mint)?;

        let weight = utils::get_weight(&token_metadata.additional_metadata)?;

        require_gt!(weight, 0, NFTManagerError::InvalidWeight);

        token_interface::burn(
            CpiContext::new(
                token_program.clone(),
                token_interface::Burn {
                    mint: mint.clone(),
                    from: source,
                    authority: signer.clone(),
                },
            ),
            amount,
        )?;

        token_interface::close_account(CpiContext::new_with_signer(
            token_program,
            token_interface::CloseAccount {
                account: mint.clone(),
                destination: signer,
                authority: mint,
            },
            mint_signer_seeds,
        ))?;

        Ok(())
    }
}
