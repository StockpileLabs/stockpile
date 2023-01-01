use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateGrant<'info> {
    #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            grant.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = grant.bump)]
    pub grant: Account<'info, Grant>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_grant(
    ctx: Context<UpdateGrant>,
    description: String,
    website_link: String,
    twitter: String,
    discord: String,
    telegram: String
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let grant = &mut ctx.accounts.grant;

    if beneficiary.key() != grant.beneficiary {
        return Err(Errors::InvalidBeneficiary.into());
    } else {
        if description.len() > 0 {
            grant.description = description;
        }

        if website_link.len() > 0 {
            grant.website_link = website_link;
        }

        if twitter.len() > 0 {
            grant.twitter = twitter;
        }

        if discord.len() > 0 {
            grant.discord = discord;
        }

        if telegram.len() > 0 {
            grant.telegram = telegram;
        }
    }

    Ok(())
}
