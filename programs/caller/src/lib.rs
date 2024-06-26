use anchor_lang::prelude::*;
use callee::cpi::accounts::CpiReturn;
use callee::program::Callee;
use callee::{ self, CpiReturnAccount };
use anchor_lang::solana_program::program_error::ProgramError;
// use std::borrow::Borrow;

declare_id!("82BDVN9BRaMZ9XkkiYkWpgzebsgNovWYbu8WCAqvemQC");

#[program]
pub mod caller {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn cpi_call_return_u64(ctx: Context<CpiReturnContext>) -> Result<()> {
        let cpi_program = ctx.accounts.cpi_return_program.to_account_info();
        let cpi_accounts = CpiReturn {
            account: ctx.accounts.cpi_return.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = callee::cpi::return_u64(cpi_ctx)?;
        let solana_return = result.get();
        anchor_lang::solana_program::log::sol_log_data(&[&solana_return.try_to_vec().unwrap()]);
        Ok(())
    }

    pub fn cpi_call_return_struct(ctx: Context<CpiReturnContext>) -> Result<()> {
        let cpi_program = ctx.accounts.cpi_return_program.to_account_info();
        let cpi_accounts = CpiReturn {
            account: ctx.accounts.cpi_return.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = callee::cpi::return_struct(cpi_ctx)?;
        let solana_return = result.get();
        anchor_lang::solana_program::log::sol_log_data(&[&solana_return.try_to_vec().unwrap()]);
        Ok(())
    }

    pub fn cpi_call_return_vec(ctx: Context<CpiReturnContext>) -> Result<()> {
        let cpi_program = ctx.accounts.cpi_return_program.to_account_info();
        let cpi_accounts = CpiReturn {
            account: ctx.accounts.cpi_return.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = callee::cpi::return_vec(cpi_ctx)?;
        let solana_return = result.get();
        if solana_return.len() > 8 {
            // Ensure you are only logging the first 8 bytes to avoid errors.
            return Err(ProgramError::InvalidArgument.into());
        }
        anchor_lang::solana_program::log::sol_log_data(&[&solana_return.try_to_vec().unwrap()]);
        Ok(())
    }

    pub fn return_u64(_ctx: Context<ReturnContext>) -> Result<u64> {
        Ok(99)
    }

    pub fn return_struct(_ctx: Context<ReturnContext>) -> Result<Struct> {
        Ok(Struct { a: 1, b: 2 })
    }

    pub fn return_vec(_ctx: Context<ReturnContext>) -> Result<Vec<u64>> {
        Ok(vec![1, 2, 3])
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Struct {
    pub a: u64,
    pub b: u64,
}

#[derive(Accounts)]
pub struct ReturnContext {}

#[derive(Accounts)]
pub struct CpiReturnContext<'info> {
    #[account(mut)]
    pub cpi_return: Account<'info, CpiReturnAccount>,
    pub cpi_return_program: Program<'info, Callee>,
}
