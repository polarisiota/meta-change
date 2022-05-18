use std::borrow::Borrow;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self};
use mpl_token_metadata::instruction::update_metadata_accounts_v2;
use mpl_token_metadata::state::{DataV2, Metadata};

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    /// CHECK:
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub update_authority: AccountInfo<'info>,

    /// CHECK:
    pub new_update_authority: AccountInfo<'info>,

    /// CHECK:
    pub token_metadata_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<UpdateAuthority>) -> Result<()> {
    let metadata_account = ctx.accounts.metadata_account.borrow();
    let update_authority = ctx.accounts.update_authority.borrow();
    // let metadata = Metadata::from_account_info(&metadata_account).unwrap();

    let ix = update_metadata_accounts_v2(
        ctx.accounts.token_metadata_program.key(),
        metadata_account.key(),
        update_authority.key(),
        Some(ctx.accounts.new_update_authority.key()),
        None,
        None,
        None,
    );

    solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.token_metadata_program.clone(),
            ctx.accounts.metadata_account.clone(),
            ctx.accounts.update_authority.clone(),
            ctx.accounts.new_update_authority.clone(),
        ],
    )?;

    Ok(())
}
