use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawCharity<'info> {
    #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            charity.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = charity.bump)]
    pub charity: Account<'info, Charity>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut, constraint = beneficiary.key() == charity.beneficiary )]
    pub beneficiary: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_charity(ctx: Context<WithdrawCharity>, amount: u64) -> Result<()> {
    let from: &mut Account<Charity> = &mut ctx.accounts.charity;
    let to: &mut UncheckedAccount = &mut ctx.accounts.beneficiary;

    let charity_goal = from.goal.parse::<u64>().unwrap();

    if from.raised < charity_goal {
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
