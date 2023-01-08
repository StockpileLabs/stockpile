use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateIndividual<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
            user_account.key().as_ref(),
            beneficiary.key().as_ref()], 
            bump, 
        payer = beneficiary, 
        space = Individual::LEN,
        )]
    pub individual: Account<'info, Individual>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn create_individual_fundraiser(
    ctx: Context<CreateIndividual>,
    name: String,
    description: String,
    image_link: String,
    twitter: String,
    discord: String,
    telegram: String,
    location: String,
    goal: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let individual = &mut ctx.accounts.individual;
    let user_account = &mut ctx.accounts.user_account;

    //let individual_goal = goal.parse::<u64>().unwrap();

    individual.raised = 0;
    individual.beneficiary = beneficiary.key();
    individual.creator = user_account.username.to_string();
    individual.name = name;
    individual.description = description;
    individual.image_link = image_link;
    individual.twitter = twitter;
    individual.telegram = telegram;
    individual.discord = discord;
    individual.location = location;
    individual.goal = goal;
    individual.contributions = 0;
    individual.bump = *ctx.bumps.get("individual").unwrap();
    individual.time = Clock::get()?.unix_timestamp;
    individual.category = "individual".to_string();

    user_account.fundraisers += 1;

    Ok(())
}