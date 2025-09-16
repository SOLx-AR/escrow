use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut/*, address = Pubkey::from_str("FhtdXoLhYtG7v5rX6d8c1b3H9g5Jz7y8x9y6z2w3v4u").unwrap() para poner una pubkey especifica */)]
    pub taker: Signer<'info>, 
    // para poner una pubkey especifica
    
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    //// deposit
    /// depositar la cantidad requerida por el maker en su cuenta, de la moneda especificada
    // definimons las cuentas involucradas en la transferencia
    // definimos el contexto del CPI
    // realizamos la transferencia
    pub fn deposit(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
           from: self.taker_ata_b.to_account_info(),
           to: self.maker_ata_b.to_account_info(),
           mint: self.mint_b.to_account_info(),
           authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    }

    //// withdraw and close
    /// el taker recibe la cantidad depositada por el maker y se cierra la boveda
    // definir las seeds que firman en nombre del escrow (quee s la autoridad de la boveda)
    // definir cuales son las cuentas involucradas en la transferencia
    // definir el contexto del CPI
    // realizar la transferencia
    // definir cuales son las cuientas involucradas en el cierre de la boveda
    // definir el contexto del CPI
    // cerramos la boveda
    pub fn withdraw_and_close(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];
        let transfer_accounts = TransferChecked {
           from: self.vault.to_account_info(),
           to: self.taker_ata_a.to_account_info(),
           mint: self.mint_a.to_account_info(),
           authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);
        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, &signer_seeds);

        close_account(ctx)
    }


}