// #region core
use anchor_lang::prelude::*;
use puppet::{Puppet, SetData};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
mod puppet_master {
    use super::*;
    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.puppet_program.clone();
        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet.clone(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, data)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut, owner = puppet_program)]
    pub puppet: Account<'info, Puppet>,
    pub puppet_program: AccountInfo<'info>,
}
// #endregion core
