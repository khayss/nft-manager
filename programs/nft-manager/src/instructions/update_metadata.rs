use anchor_lang::{prelude::*, system_program as system};
use anchor_spl::{
    token_2022::{
        spl_token_2022::{
            extension::{BaseStateWithExtensions, PodStateWithExtensions},
            pod::PodMint,
        },
        Token2022,
    },
    token_interface,
};
use spl_token_metadata_interface::state::TokenMetadata;

use crate::{
    constants::{MINT_TAG, NFT_MANAGER_TAG},
    errors::NFTManagerError,
    states::nft_manager::NFTManager,
};

#[derive(Accounts)]
#[instruction(args: UpdateMetadataArgs)]
pub struct UpdateMetadata<'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,

    #[account(
        mut,
        address = nft_manager.authority @NFTManagerError::OnlyAdminAllowed,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [MINT_TAG, &args.discriminant.to_le_bytes()],
        bump,
    )]
    pub mint: InterfaceAccount<'info, token_interface::Mint>,

    #[account(
        seeds = [NFT_MANAGER_TAG],
        bump,
    )]
    pub nft_manager: Account<'info, NFTManager>,
}

impl<'info> UpdateMetadata<'info> {
    pub fn update_metadata(
        &mut self,
        bumps: &UpdateMetadataBumps,
        args: UpdateMetadataArgs,
    ) -> Result<()> {
        let mint = self.mint.to_account_info();
        let token_program = self.token_program.to_account_info();
        let system_program = self.system_program.to_account_info();
        let signer = self.admin.to_account_info();

        let (current_lamports, required_lamports) = {
            let buffer = mint.try_borrow_data()?;
            let state = PodStateWithExtensions::<PodMint>::unpack(&buffer)?;

            // Get and update the token metadata
            let mut token_metadata = state.get_variable_len_extension::<TokenMetadata>()?;
            token_metadata.update(args.field.clone().into(), args.value.clone());

            // Calculate the new account length with the updated metadata
            let new_account_len =
                state.try_get_new_account_len_for_variable_len_extension(&token_metadata)?;

            // Calculate the required lamports for the new account length
            let required_lamports = Rent::get()?.minimum_balance(new_account_len);
            // Get the current lamports of the mint account
            let current_lamports = mint.lamports();

            (current_lamports, required_lamports)
        };

        // Transfer lamports to mint account for the additional metadata if needed
        if required_lamports > current_lamports {
            let lamport_difference = required_lamports - current_lamports;
            system::transfer(
                CpiContext::new(
                    system_program,
                    system::Transfer {
                        from: signer,
                        to: mint.clone(),
                    },
                ),
                lamport_difference,
            )?;
        }

        let mint_seeds = &[MINT_TAG, &args.discriminant.to_le_bytes(), &[bumps.mint]];
        let mint_signer_seeds = &[&mint_seeds[..]];

        token_interface::token_metadata_update_field(
            CpiContext::new_with_signer(
                token_program.clone(),
                token_interface::TokenMetadataUpdateField {
                    token_program_id: token_program.clone(),
                    metadata: mint.clone(),
                    update_authority: mint,
                },
                mint_signer_seeds,
            ),
            args.field.clone().into(),
            args.value.clone(),
        )?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum TokenMetadataFields {
    Name,
    Symbol,
    Uri,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMetadataArgs {
    pub field: TokenMetadataFields,
    pub value: String,
    pub discriminant: u64,
}

impl From<TokenMetadataFields> for token_interface::spl_token_metadata_interface::state::Field {
    fn from(value: TokenMetadataFields) -> Self {
        match value {
            TokenMetadataFields::Name => {
                token_interface::spl_token_metadata_interface::state::Field::Name
            }
            TokenMetadataFields::Symbol => {
                token_interface::spl_token_metadata_interface::state::Field::Symbol
            }
            TokenMetadataFields::Uri => {
                token_interface::spl_token_metadata_interface::state::Field::Uri
            }
        }
    }
}
