use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawIndividual<'info> {
    #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            individual.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = individual.bump)]
    pub individual: Account<'info, Individual>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut, constraint = beneficiary.key() == individual.beneficiary )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub beneficiary: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_individual(ctx: Context<WithdrawIndividual>, amount: u64) -> Result<()> {
    let from: &mut Account<Individual> = &mut ctx.accounts.individual;
    let to: &mut UncheckedAccount = &mut ctx.accounts.beneficiary;

    let individual_goal = from.goal.parse::<u64>().unwrap();

    if from.raised < individual_goal {
        return Err(Errors::GoalNotMet.into());
    }

    if to.key() != from.beneficiary {
        return Err(Errors::InvalidBeneficiary.into());
    }

    **from.to_account_info().try_borrow_mut_lamports()? -= amount as u64;
    **to.try_borrow_mut_lamports()? += amount as u64;

    from.balance -= amount;

    Ok(())
}
