use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GooUZvk7JsXpggyBne9UStxV4RJkuevKzmPyhCN29kGW");

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
