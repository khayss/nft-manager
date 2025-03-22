use anchor_lang::{
    prelude::*,
    solana_program::rent::{DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR},
    system_program as system,
};
use anchor_spl::{token_2022::Token2022, token_interface};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

use crate::constants::{COLLECTION_TAG, COLLECTION_TOKEN_ACCOUNT_TAG};

#[derive(Accounts)]

pub struct CreateCollection<'info> {
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [COLLECTION_TAG],
        bump,
        mint::decimals = 0,
        mint::authority = mint,
        mint::freeze_authority = mint,
        extensions::metadata_pointer::metadata_address = mint,
        extensions::metadata_pointer::authority = mint,
        extensions::group_pointer::authority = mint,
        extensions::group_pointer::group_address = mint,
    )]
    // collection mint
    pub mint: InterfaceAccount<'info, token_interface::Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [COLLECTION_TOKEN_ACCOUNT_TAG],
        bump,
        token::mint = mint,
        token::authority = token_account,
    )]
    // collection token account
    pub token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
}

impl<'info> CreateCollection<'info> {
    pub fn create_collection(
        &mut self,
        bumps: &CreateCollectionBumps,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        let destination = self.token_account.to_account_info();
        let mint = self.mint.to_account_info();
        let token_program = self.token_program.to_account_info();

        let mint_seeds = &[COLLECTION_TAG, &[bumps.mint]];
        let mint_signer_seeds: &[&[&[u8]]] = &[&mint_seeds[..]];

        let token_metadata = TokenMetadata {
            name: args.name.clone(),
            symbol: args.symbol.clone(),
            uri: args.uri.clone(),
            ..Default::default()
        };

        let data_len = 4 + token_metadata.get_packed_len()?;

        let lamports =
            data_len as u64 * DEFAULT_LAMPORTS_PER_BYTE_YEAR * DEFAULT_EXEMPTION_THRESHOLD as u64;

        system::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                system::Transfer {
                    from: self.signer.to_account_info(),
                    to: mint.clone(),
                },
            ),
            lamports,
        )?;

        token_interface::token_metadata_initialize(
            CpiContext::new_with_signer(
                token_program.clone(),
                token_interface::TokenMetadataInitialize {
                    token_program_id: token_program.clone(),
                    mint: mint.clone(),
                    metadata: mint.clone(),
                    mint_authority: mint.clone(),
                    update_authority: mint.clone(),
                },
                mint_signer_seeds,
            ),
            args.name,
            args.symbol,
            args.uri,
        )?;

        token_interface::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token_interface::MintTo {
                    mint: mint.clone(),
                    to: destination.clone(),
                    authority: mint.clone(),
                },
                mint_signer_seeds,
            ),
            1,
        )?;

        token_interface::set_authority(
            CpiContext::new_with_signer(
                token_program,
                token_interface::SetAuthority {
                    current_authority: mint.clone(),
                    account_or_mint: mint.clone(),
                },
                mint_signer_seeds,
            ),
            token_interface::spl_token_2022::instruction::AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
