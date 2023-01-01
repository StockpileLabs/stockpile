use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateCharity<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
            user_account.key().as_ref(),
            beneficiary.key().as_ref()], 
            bump, 
        payer = beneficiary, 
        space = Charity::LEN,
        )]
    pub charity: Account<'info, Charity>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn create_charity_fundraiser(
    ctx: Context<CreateCharity>,
    name: String,
    description: String,
    image_link: String,
    website_link: String,
    twitter: String,
    discord: String,
    telegram: String,
    location: String,
    filing_link: String,
    goal: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let charity = &mut ctx.accounts.charity;
    let user_account = &mut ctx.accounts.user_account;

    //let charity_goal = goal.parse::<u64>().unwrap();

    charity.raised = 0;
    charity.beneficiary = beneficiary.key();
    charity.creator = user_account.username.to_string();
    charity.name = name;
    charity.description = description;
    charity.image_link = image_link;
    charity.twitter = twitter;
    charity.discord = discord;
    charity.telegram = telegram;
    charity.location = location;
    charity.website_link = website_link;
    charity.filing_link = filing_link;
    charity.goal = goal;
    charity.contributions = 0;
    charity.bump = *ctx.bumps.get("charity").unwrap();
    charity.time = Clock::get()?.unix_timestamp;
    charity.category = "charity".to_string();

    user_account.fundraisers += 1;

    Ok(())
}
