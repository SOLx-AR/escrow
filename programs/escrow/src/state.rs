use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
    pub clock: i64,
}

#[error_code]
pub enum EscrowError {
    #[msg("The amount must be greater than zero")]
    InvalidAmount,
    #[msg("The amount of deposit must be equal to the amount to receive")]
    AmountMismatch,
    #[msg("The current time must be greater than the escrow creation time")]
    InvalidTime,
}