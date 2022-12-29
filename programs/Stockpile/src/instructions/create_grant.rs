use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String, repo: String)]
pub struct CreateGrant<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
            repo.as_ref(),
            user_account.key().as_ref(),
            beneficiary.key().as_ref()], 
            bump, 
        payer = beneficiary, 
        space = Grant::LEN,
        )]
    pub grant: Account<'info, Grant>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn create_grant_fundraiser(
    ctx: Context<CreateGrant>,
    name: String,
    description: String,
    image_link: String,
    website_link: String,
    contact_link: String,
    repo: String,
    goal: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let grant = &mut ctx.accounts.grant;
    let user_account = &mut ctx.accounts.user_account;

    let grant_goal = goal.parse::<u64>().unwrap();

    grant.raised = 0;
    grant.beneficiary = beneficiary.key();
    grant.creator = user_account.username.to_string();
    grant.name = name;
    grant.description = description;
    grant.image_link = image_link;
    grant.contact_link = contact_link;
    grant.website_link = website_link;
    grant.repo = repo;
    grant.goal = grant_goal.to_string();
    grant.contributions = 0;
    grant.bump = *ctx.bumps.get("grant").unwrap();
    grant.time = Clock::get()?.unix_timestamp;
    grant.category = "grant".to_string();

    user_account.fundraisers += 1;

    Ok(())
}
