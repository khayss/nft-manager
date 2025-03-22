use anchor_lang::prelude::*;

use crate::MintNFTArgs;

#[account]
pub struct FinalizeFractionalizeData {
    pub mint: Pubkey,
    pub weight: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub bump: u8,
}

impl FinalizeFractionalizeData {
    pub fn calculate_space(part_b: MintNFTArgs) -> usize {
        let name_b = 4 + part_b.name.as_bytes().len();
        let symbol_b = 4 + part_b.symbol.as_bytes().len();
        let uri_b = 4 + part_b.uri.as_bytes().len();

        name_b + symbol_b + uri_b + 41
    }

    pub fn init(&mut self, mint: Pubkey, part_b: MintNFTArgs, bump: u8) -> Result<()> {
        self.mint = mint;
        self.weight = part_b.weight;
        self.name = part_b.name;
        self.symbol = part_b.symbol;
        self.uri = part_b.uri;
        self.bump = bump;

        Ok(())
    }
}
