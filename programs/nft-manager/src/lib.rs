use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

pub use instructions::*;

declare_id!("78TGdayzTnEPi8UVMeRgJYSx6uawNB3CHTrcBBMM2gDK");

#[program]
pub mod nft_manager {
    use super::*;

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        ctx.accounts.create_collection(&ctx.bumps, args)?;

        Ok(())
    }

    pub fn initialize_nft_manager(
        ctx: Context<InitializeNFTManager>,
        args: InitializeNFTManagerArgs,
    ) -> Result<()> {
        ctx.accounts.initialize_nft_manager(&ctx.bumps, args)?;
        Ok(())
    }

    pub fn update_fees(ctx: Context<UpdateFees>, args: UpdateFeesArgs) -> Result<()> {
        ctx.accounts.update_fees(args)?;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNFT>, args: MintNFTArgs) -> Result<()> {
        ctx.accounts.mint_nft(&ctx.bumps, args)?;
        Ok(())
    }

    pub fn finalize_mint_nft(ctx: Context<FinalizeMintNFT>, discriminant: u64) -> Result<()> {
        ctx.accounts.finalize_mint_nft(&ctx.bumps, discriminant)?;
        Ok(())
    }

    pub fn fractionalize_nft(
        ctx: Context<FractionalizeNFT>,
        args: FractionalizeNFTArgs,
    ) -> Result<()> {
        ctx.accounts.fractionalize_nft(&ctx.bumps, args)?;
        Ok(())
    }

    pub fn finalize_fractionalize_nft(
        ctx: Context<FinalizeFractionalizeNFT>,
        _discriminant: u64,
    ) -> Result<()> {
        ctx.accounts.finalize_mint_nft(&ctx.bumps)?;
        Ok(())
    }

    pub fn burn_nft(ctx: Context<BurnNFT>, dicriminant: u64) -> Result<()> {
        ctx.accounts.burn_nft(&ctx.bumps, dicriminant)?;
        Ok(())
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, args: UpdateMetadataArgs) -> Result<()> {
        ctx.accounts.update_metadata(&ctx.bumps, args)?;

        Ok(())
    }

    pub fn list_nft(ctx: Context<ListNFT>, args: ListNFTArgs) -> Result<()> {
        ctx.accounts.list_nft(&ctx.bumps, args)?;

        Ok(())
    }

    pub fn delist_nft(ctx: Context<DelistNFT>, _dicriminant: u64) -> Result<()> {
        ctx.accounts.delist_nft(&ctx.bumps)?;

        Ok(())
    }

    pub fn update_listing_price(
        ctx: Context<UpdateListingPrice>,
        args: UpdateListingPriceArgs,
    ) -> Result<()> {
        ctx.accounts.update_listing_price(args)?;

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, _discriminant: u64) -> Result<()> {
        ctx.accounts.buy_nft(&ctx.bumps)?;
        Ok(())
    }

    pub fn admin_withdraw_fees(ctx: Context<AdminWithdrawFees>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_fees(amount)?;
        Ok(())
    }

    pub fn admin_withdraw_mint_fees(
        ctx: Context<AdminWithdrawMintFees>,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.withdraw_mint_fees(amount)?;

        Ok(())
    }

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        ctx.accounts.create_user_account(&ctx.bumps)?;
        Ok(())
    }

    pub fn user_withdraw(ctx: Context<UserWithdraw>, amount: u64) -> Result<()> {
        ctx.accounts.user_withdraw(amount)?;

        Ok(())
    }

    pub fn initiailize_ownership_transfer(ctx: Context<InitializeOwnershipTransfer>) -> Result<()> {
        ctx.accounts.initialize_transfer_ownership()?;
        Ok(())
    }

    pub fn finalize_ownership_transfer(ctx: Context<FinalizeOwnershipTransfer>) -> Result<()> {
        ctx.accounts.finalize_ownership_transfer()?;
        Ok(())
    }
}
