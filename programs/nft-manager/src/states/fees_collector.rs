use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FeesCollector {
    pub bump: u8,
    pub fractionalize_fee: u32,
    pub sell_fee: u32,
    pub fees_decimals: u8,
}

impl FeesCollector {
    pub fn init(&mut self, fractionalize_fee: u32, sell_fee: u32, bump: u8) -> Result<()> {
        self.bump = bump;
        self.fractionalize_fee = fractionalize_fee;
        self.sell_fee = sell_fee;
        self.fees_decimals = 4;

        Ok(())
    }
}
