use anchor_lang::prelude::*;

pub mod state;
use state::*;

pub mod contexts;
use contexts::*;

declare_id!("FqPyd6hhBjhk98jwkQkbRvzkSVPhiXVzoKgZ5mSU4TQ8");

#[program]
pub mod escrow {
    use super::*;

    // un usuario crea un escrow depositando una cierta cantidad de una criptomoneda
    // y define que moneda y cantidad quiere recivir a cambio
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive:u64) -> Result<()> {
        ctx.accounts.deposit(deposit);
        ctx.accounts.save_escrow(seed, receive, &ctx.bumps)?;
        Ok(())
    }

    // otro usuario acepta el escrow y deposita la cantidad y moneda requerida
    // el usuario que crea el escrow recibe la cantidad y moneda que definio
   /* pub fn take(ctx: Context<Take>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }


    // el usuario creado cancela el escrow
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    */
}
