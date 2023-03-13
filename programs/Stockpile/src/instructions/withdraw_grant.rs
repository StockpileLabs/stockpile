use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawGrant<'info> {
    #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            grant.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = grant.bump)]
    pub grant: Account<'info, Grant>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut, constraint = beneficiary.key() == grant.beneficiary )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub beneficiary: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_grant(ctx: Context<WithdrawGrant>, amount: u64) -> Result<()> {
    let from: &mut Account<Grant> = &mut ctx.accounts.grant;
    let to: &mut UncheckedAccount = &mut ctx.accounts.beneficiary;

    let grant_goal = from.goal.parse::<u64>().unwrap();

    if from.balance < amount {
        return Err(Errors::AmountTooLarge.into());
    }

    if from.raised < grant_goal {
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
