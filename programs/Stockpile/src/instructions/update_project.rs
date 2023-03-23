use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateProject<'info> {
    #[account(mut, 
        has_one = treasury, 
        seeds = [
            project.name.as_ref(), 
            user_account.key().as_ref(), 
            treasury.key().as_ref()],
        bump = project.bump)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub treasury: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_project(
    ctx: Context<UpdateProject>,
    description: String,
    website_link: String,
    twitter: String,
    discord: String,
    telegram: String,
    repo: String,
    image: String,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let project = &mut ctx.accounts.project;

    if treasury.key() != project.treasury {
        return Err(Errors::InvalidBeneficiary.into());
    } else {
        if description.len() > 0 {
            project.description = description;
        }

        if website_link.len() > 0 {
            project.website_link = website_link;
        }

        if twitter.len() > 0 {
            project.twitter = twitter;
        }

        if telegram.len() > 0 {
            project.telegram = telegram;
        }

        if discord.len() > 0 {
            project.discord = discord;
        }

        if repo.len() > 0 {
            project.repo = repo;
        }

        if image.len() > 0 {
            project.image_link = image;
        }
    }

    project.last_updated = Clock::get()?.unix_timestamp;

    Ok(())
}
