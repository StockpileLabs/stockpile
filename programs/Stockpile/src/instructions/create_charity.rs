use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock;

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
        has_one = beneficiary,
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
    contact_link: String,
    filing_link: String,
    goal: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let charity = &mut ctx.accounts.charity;
    let user_account = &mut ctx.accounts.user_account;

    let charity_goal = goal.parse::<u64>().unwrap();

    charity.raised = 0;
    charity.beneficiary = beneficiary.key();
    charity.creator = user_account.username.to_string();
    charity.name = name;
    charity.description = description;
    charity.image_link = image_link;
    charity.contact_link = contact_link;
    charity.website_link = website_link;
    charity.filing_link = filing_link;
    charity.goal = charity_goal.to_string();
    charity.contributions = 0;
    charity.bump = *ctx.bumps.get("charity").unwrap();
    charity.time = Clock::get()?.unix_timestamp;
    charity.category = "charity".to_string();

    user_account.fundraisers += 1;

    Ok(())
}
