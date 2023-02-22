use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGrant<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
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
    twitter: String,
    discord: String,
    telegram: String,
    location: String,
    repo: String,
    goal: String,
    is_matching_eligible: bool,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let grant = &mut ctx.accounts.grant;
    let user_account = &mut ctx.accounts.user_account;

    //let grant_goal = goal.parse::<u64>().unwrap();

    grant.raised = 0;
    grant.beneficiary = beneficiary.key();
    grant.creator = user_account.username.to_string();
    grant.name = name;
    grant.description = description;
    grant.image_link = image_link;
    grant.website_link = website_link;
    grant.twitter = twitter;
    grant.discord = discord;
    grant.telegram = telegram;
    grant.location = location;
    grant.repo = repo;
    grant.goal = goal;
    grant.contributions = 0;
    grant.balance = 0;
    grant.last_updated = 0;
    grant.is_matching_eligible = is_matching_eligible;
    grant.bump = *ctx.bumps.get("grant").unwrap();
    grant.time = Clock::get()?.unix_timestamp;
    grant.category = "grant".to_string();

    user_account.fundraisers += 1;

    Ok(())
}
