use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod instructions;
#[program]
pub mod meta_change {
    use super::*;

    pub fn update_authority(ctx: Context<UpdateAuthority>) -> Result<()> {
        instructions::update_authority::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
