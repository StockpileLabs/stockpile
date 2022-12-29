use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateProject<'info> {
    #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            project.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = project.bump)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_project(
    ctx: Context<UpdateProject>,
    description: String,
    website_link: String,
    contact_link: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let project = &mut ctx.accounts.project;

    if beneficiary.key() != project.beneficiary {
        return Err(Errors::InvalidBeneficiary.into());
    } else {
        if description.len() > 0 {
            project.description = description;
        }

        if contact_link.len() > 0 {
            project.contact_link = contact_link;
        }

        if website_link.len() > 0 {
            project.website_link = website_link;
        }
    }

    Ok(())
}
