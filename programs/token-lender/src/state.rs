use anchor_lang::prelude::*;

#[account]
pub struct LoanEscrow {
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub deposit: u64, // USDC
    pub expiry_timestamp: u64,
    pub loan_id: u8,
    pub bump: u8,
}

impl LoanEscrow {
    pub const SEED_PREFIX: &'static str = "loan_escrow";

    pub fn is_expired(&self, current_slot: u64) -> bool {
        current_slot > self.expiry_timestamp
    }

    pub fn is_fully_paid(&self, usdc_ata_amount: u64) -> bool {
        usdc_ata_amount >= self.deposit
    }
}