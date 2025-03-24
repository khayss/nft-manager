use anchor_lang::{
    prelude::*,
    solana_program::{
        native_token::LAMPORTS_PER_SOL,
        rent::{DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR},
    },
    system_program as system,
};
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{BaseStateWithExtensions, PodStateWithExtensions},
        pod::PodMint,
    },
    token_interface,
};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, Price, PriceUpdateV2};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

use crate::{
    constants::{COLLECTION_KEY, DISCRIMINANT_KEY, MAX_AGE, SOL_PRICE_FEED_ID_HEX, WEIGHT_KEY},
    errors::NFTManagerError,
    states::nft_manager::NFTManager,
};

// #[inline(always)]
pub fn calc_gold_value_in_lamport(gold_price: Price, sol_price: Price, weight: u64) -> Result<u64> {
    // Ensure weight is non-zero to avoid unnecessary computations
    require_gte!(weight, 0, NFTManagerError::InvalidWeight);
    require_gt!(gold_price.price, 0, NFTManagerError::NegativePrice);
    require_gt!(sol_price.price, 0, NFTManagerError::NegativePrice);

    let mut numerator = gold_price.price as u128;
    let mut denominator = sol_price.price as u128;

    numerator = numerator
        .checked_add(gold_price.conf as u128)
        .and_then(|v| v.checked_mul(weight as u128))
        .and_then(|v| v.checked_mul(LAMPORTS_PER_SOL as u128))
        .ok_or(NFTManagerError::Overflow)?;

    denominator = denominator
        .checked_sub(sol_price.conf as u128)
        .and_then(|v| v.checked_mul(2_83))
        .ok_or(NFTManagerError::Overflow)?;

    if gold_price.exponent > sol_price.exponent {
        numerator = numerator
            .checked_mul(10u128.pow((gold_price.exponent - sol_price.exponent) as u32))
            .ok_or(NFTManagerError::Overflow)?;
    } else {
        denominator = denominator
            .checked_mul(10u128.pow((sol_price.exponent - gold_price.exponent) as u32))
            .ok_or(NFTManagerError::Overflow)?;
    }

    let lamport = numerator
        .checked_div(denominator)
        .ok_or(NFTManagerError::Overflow)?;

    require_gt!(lamport, 0, NFTManagerError::PriceCalculationFail);

    Ok(lamport as u64)
}

#[inline(always)]
pub fn get_gold_value_in_lamport(
    gold_price_update: &Account<PriceUpdateV2>,
    sol_price_update: &Account<PriceUpdateV2>,
    weight: u64,
) -> Result<u64> {
    let gold_feed_id: [u8; 32] =
        get_feed_id_from_hex("765d2ba906dbc32ca17cc11f5310a89e9ee1f6420508c63861f2f8ba4ee34bb2")?;
    let sol_feed_id: [u8; 32] = get_feed_id_from_hex(SOL_PRICE_FEED_ID_HEX)?;
    let gold_price =
        gold_price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &gold_feed_id)?;
    let sol_price =
        sol_price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &sol_feed_id)?;

    calc_gold_value_in_lamport(gold_price, sol_price, weight)
}

pub fn validate_weight(additional_metadata: &[(String, String)]) -> Result<()> {
    let weight = additional_metadata
        .iter()
        .find(|(key, _)| key == WEIGHT_KEY)
        .ok_or(NFTManagerError::InvalidMetadata)?
        .1
        .parse::<u64>()
        .map_err(|_| NFTManagerError::InvalidMetadata)?;

    require_gt!(weight, 0, NFTManagerError::InvalidWeight);

    Ok(())
}

pub fn validate_fractions(
    weight: u64,
    fraction_a_weight: u64,
    fraction_b_weight: u64,
) -> Result<()> {
    require_eq!(
        weight,
        fraction_a_weight + fraction_b_weight,
        NFTManagerError::InvalidWeight
    );

    require_gte!(fraction_a_weight, 1, NFTManagerError::InvalidWeight);
    require_gte!(fraction_b_weight, 1, NFTManagerError::InvalidWeight);

    Ok(())
}

pub fn get_weight(additional_metadata: &[(String, String)]) -> Result<u64> {
    let weight = additional_metadata
        .iter()
        .find(|(key, _)| key == WEIGHT_KEY)
        .ok_or(NFTManagerError::InvalidMetadata)?
        .1
        .parse::<u64>()
        .map_err(|_| NFTManagerError::InvalidMetadata)?;

    Ok(weight)
}

pub fn validate_collection(
    additional_metadata: &[(String, String)],
    collection: Pubkey,
) -> Result<()> {
    let collection_key = additional_metadata
        .iter()
        .find(|(key, _)| key == COLLECTION_KEY)
        .ok_or(NFTManagerError::InvalidMetadata)?
        .1
        .parse::<Pubkey>()
        .map_err(|_| NFTManagerError::InvalidMetadata)?;

    require_eq!(
        collection,
        collection_key,
        NFTManagerError::InvalidCollection
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn mint_nft<'a>(
    name: String,
    symbol: String,
    uri: String,
    weight: u64,
    discriminant: u64,
    mint_signer_seeds: &[&[&[u8]]],
    nft_manager: &mut NFTManager,
    mint: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    payer: AccountInfo<'a>,
) -> Result<()> {
    nft_manager.increment_discriminant()?;

    require_gte!(weight, 0, NFTManagerError::InvalidWeight);

    let token_metadata = TokenMetadata {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        ..Default::default()
    };

    let data_len = 4 + token_metadata.get_packed_len()?;

    let lamports =
        data_len as u64 * DEFAULT_LAMPORTS_PER_BYTE_YEAR * DEFAULT_EXEMPTION_THRESHOLD as u64;

    system::transfer(
        CpiContext::new(
            system_program.clone(),
            system::Transfer {
                from: payer.clone(),
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
        name,
        symbol,
        uri,
    )?;

    let (current_lamports, required_lamports) = {
        let buffer = mint.try_borrow_data()?;
        let state = PodStateWithExtensions::<PodMint>::unpack(&buffer)?;

        let mut token_metadata = state.get_variable_len_extension::<TokenMetadata>()?;
        token_metadata.update(
            token_interface::spl_token_metadata_interface::state::Field::Key(
                DISCRIMINANT_KEY.to_string(),
            ),
            discriminant.to_string(),
        );

        let new_account_len =
            state.try_get_new_account_len_for_variable_len_extension(&token_metadata)?;

        let required_lamports = Rent::get()?.minimum_balance(new_account_len);

        let current_lamports = mint.lamports();

        (current_lamports, required_lamports)
    };

    if required_lamports > current_lamports {
        let lamport_difference = required_lamports - current_lamports;
        system::transfer(
            CpiContext::new(
                system_program,
                system::Transfer {
                    from: payer,
                    to: mint.clone(),
                },
            ),
            lamport_difference,
        )?;
    }

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint.clone(),
            },
            mint_signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Key(
            DISCRIMINANT_KEY.to_string(),
        ),
        discriminant.to_string(),
    )?;

    token_interface::mint_to(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::MintTo {
                mint: mint.clone(),
                to: destination,
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

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn update_metadata<'a>(
    token_program: AccountInfo<'a>,
    mint: AccountInfo<'a>,
    mint_signer_seeds: &[&[&[u8]]],
    weight: u64,
    name: &str,
    symbol: &str,
    uri: &str,
    collection: &str,
) -> Result<()> {
    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint.clone(),
            },
            mint_signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Key(WEIGHT_KEY.to_string()),
        weight.to_string(),
    )?;

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint.clone(),
            },
            mint_signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Name,
        name.to_string(),
    )?;

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint.clone(),
            },
            mint_signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Symbol,
        symbol.to_string(),
    )?;

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint.clone(),
            },
            mint_signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Uri,
        uri.to_string(),
    )?;

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program,
                metadata: mint.clone(),
                update_authority: mint,
            },
            mint_signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Key(
            COLLECTION_KEY.to_string(),
        ),
        collection.to_string(),
    )?;

    Ok(())
}

#[inline(always)]
pub fn update_metadata_standard<'a>(
    mint: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    collection: Pubkey,
    weight: u64,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let (current_lamports, required_lamports) = {
        let buffer = mint.try_borrow_data()?;
        let state = PodStateWithExtensions::<PodMint>::unpack(&buffer)?;

        // Get and update the token metadata
        let mut token_metadata = state.get_variable_len_extension::<TokenMetadata>()?;
        token_metadata.update(
            token_interface::spl_token_metadata_interface::state::Field::Key(
                WEIGHT_KEY.to_string(),
            ),
            weight.to_string(),
        );

        token_metadata.update(
            token_interface::spl_token_metadata_interface::state::Field::Key(
                COLLECTION_KEY.to_string(),
            ),
            collection.to_string(),
        );

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
                system_program.clone(),
                system::Transfer {
                    from: payer.clone(),
                    to: mint.clone(),
                },
            ),
            lamport_difference,
        )?;
    }

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint.clone(),
            },
            signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Key(WEIGHT_KEY.to_string()),
        weight.to_string(),
    )?;

    token_interface::token_metadata_update_field(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TokenMetadataUpdateField {
                token_program_id: token_program.clone(),
                metadata: mint.clone(),
                update_authority: mint,
            },
            signer_seeds,
        ),
        token_interface::spl_token_metadata_interface::state::Field::Key(
            COLLECTION_KEY.to_string(),
        ),
        collection.to_string(),
    )?;

    Ok(())
}

pub fn get_metadata(mint: &AccountInfo) -> Result<TokenMetadata> {
    let buffer = mint.try_borrow_data()?;
    let state = PodStateWithExtensions::<PodMint>::unpack(&buffer)?;

    // Get and update the token metadata
    Ok(state.get_variable_len_extension::<TokenMetadata>()?)
}

#[cfg(test)]
mod test {
    use super::*;

    const PUBLISH_TIME: i64 = 1000000;
    const WEIGHT: u64 = 10;

    #[test]
    fn test_all_negative_exponents() {
        let sol_price = 13466877236;
        let sol_conf = 9965337;
        let sol_exponent = -8;

        let gold_price = 2989990;
        let gold_conf = 1173;
        let gold_exponent = -3;

        let sol_p = Price {
            price: sol_price,
            conf: sol_conf,
            exponent: sol_exponent,
            publish_time: PUBLISH_TIME,
        };

        let gold_p = Price {
            price: gold_price,
            conf: gold_conf,
            exponent: gold_exponent,
            publish_time: PUBLISH_TIME,
        };

        let price_1 = calc_gold_value_in_lamport(gold_p, sol_p, WEIGHT).unwrap();
        let price_2 = calc_gold_value_in_lamport(gold_p, sol_p, WEIGHT).unwrap();

        println!("Price 1: {}", price_1);

        assert_eq!(price_1, price_2, "Prices should be equal");
    }
}
