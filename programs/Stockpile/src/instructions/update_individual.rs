use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateIndividual<'info> {
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
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_individual(
    ctx: Context<UpdateIndividual>,
    description: String,
    twitter: String,
    discord: String,
    telegram: String,
    image: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let individual = &mut ctx.accounts.individual;

    if beneficiary.key() != individual.beneficiary {
        return Err(Errors::InvalidBeneficiary.into());
    } else {
        if description.len() > 0 {
            individual.description = description;
        }

        if twitter.len() > 0 {
            individual.twitter = twitter;
        }

        if telegram.len() > 0 {
            individual.telegram = telegram;
        }

        if discord.len() > 0 {
            individual.discord = discord;
        }

        if image.len() > 0 {
            individual.image_link = image;
        }

    }

    individual.last_updated = Clock::get()?.unix_timestamp;

    Ok(())
}
