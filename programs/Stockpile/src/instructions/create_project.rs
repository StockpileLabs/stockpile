use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateProject<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
            user_account.key().as_ref(),
            treasury.key().as_ref()], 
            bump, 
        payer = treasury, 
        space = Project::LEN,
        )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub treasury: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn create_project_fundraiser(
    ctx: Context<CreateProject>,
    name: String,
    description: String,
    image_link: String,
    website_link: String,
    contact_link: String,
    repo: String,
    goal: String,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let project = &mut ctx.accounts.project;
    let user_account = &mut ctx.accounts.user_account;

    let project_goal = goal.parse::<u64>().unwrap();

    project.raised = 0;
    project.treasury = treasury.key();
    project.creator = user_account.username.to_string();
    project.name = name;
    project.description = description;
    project.image_link = image_link;
    project.contact_link = contact_link;
    project.website_link = website_link;
    project.repo = repo;
    project.goal = project_goal.to_string();
    project.contributions = 0;
    project.bump = *ctx.bumps.get("project").unwrap();
    project.time = Clock::get()?.unix_timestamp;
    project.category = "project".to_string();

    user_account.fundraisers += 1;

    Ok(())
}
