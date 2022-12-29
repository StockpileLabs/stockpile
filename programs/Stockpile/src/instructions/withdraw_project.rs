use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawProject<'info> {
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
    #[account(mut, constraint = beneficiary.key() == project.beneficiary )]
    pub beneficiary: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_project(ctx: Context<WithdrawProject>, amount: u64) -> Result<()> {
    let from: &mut Account<Project> = &mut ctx.accounts.project;
    let to: &mut UncheckedAccount = &mut ctx.accounts.beneficiary;

    let project_goal = from.goal.parse::<u64>().unwrap();

    if from.raised < project_goal {
        return Err(Errors::GoalNotMet.into());
    }

    if to.key() != from.beneficiary {
        return Err(Errors::InvalidBeneficiary.into());
    }

    **from.to_account_info().try_borrow_mut_lamports()? -= amount as u64;
    **to.try_borrow_mut_lamports()? += amount as u64;

    from.raised -= amount;

    Ok(())
}
