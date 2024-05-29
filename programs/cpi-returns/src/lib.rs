use anchor_lang::prelude::*;

declare_id!("H769wqK45ff6TpatUvJ5Q4ehAMioQ6oNM2SeAjqwahJ6");

#[program]
pub mod cpi_returns {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
