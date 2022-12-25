#![feature(derive_default_enum)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar::clock;

declare_id!("STKqgkK72R1sJhV4BTipUcKkMFougtdTuDMr2RVubKr");

#[program]
pub mod stockpile {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.username = username;
        user_account.bump = *ctx.bumps.get("user_account").unwrap();
        user_account.authority = authority.key();
        user_account.time = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, username: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.username = username;

        Ok(())
    }

    pub fn create_fundraiser(
        ctx: Context<CreateFundraiser>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        contact_link: String,
        goal: String,
        category: Category,
    ) -> Result<()> {
        let beneficiary = &mut ctx.accounts.beneficiary;
        let fundraiser = &mut ctx.accounts.fundraiser;
        let user_account = &mut ctx.accounts.user_account;

        let fundraiser_goal = goal.parse::<u64>().unwrap();

        fundraiser.raised = 0;
        fundraiser.beneficiary = beneficiary.key();
        fundraiser.creator = user_account.username.to_string();
        fundraiser.name = name;
        fundraiser.description = description;
        fundraiser.image_link = image_link;
        fundraiser.contact_link = contact_link;
        fundraiser.website_link = website_link;
        fundraiser.goal = (fundraiser_goal * 100).to_string();
        fundraiser.contributions = 0;
        fundraiser.bump = *ctx.bumps.get("fundraiser").unwrap();
        fundraiser.time = Clock::get()?.unix_timestamp;
        fundraiser.category = category;

        user_account.fundraisers += 1;

        Ok(())
    }

    pub fn update_fundraiser(
        ctx: Context<UpdateFundraiser>,
        description: String,
        website_link: String,
        contact_link: String,
    ) -> Result<()> {
        let beneficiary = &mut ctx.accounts.beneficiary;
        let fundraiser = &mut ctx.accounts.fundraiser;

        if beneficiary.key() != fundraiser.beneficiary {
            return Err(Errors::InvalidAuthority.into());
        }

        if description.len() > 0 {
            fundraiser.description = description;
        }

        if contact_link.len() > 0 {
            fundraiser.contact_link = contact_link;
        }

        if website_link.len() > 0 {
            fundraiser.website_link = website_link;
        }

        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: f64) -> Result<()> {
        let fundraiser = &mut ctx.accounts.fundraiser;
        let contributor = &mut ctx.accounts.contributor;
        let user_account = &mut ctx.accounts.user_account;

        let pseudo_lamports = amount * 100 as f64;

        fundraiser.raised += pseudo_lamports as u64;
        fundraiser.contributions += 1;
        user_account.contributions += 1;

        Ok(())
    }

    pub fn fundraiser_withdraw(ctx: Context<FundraiserWithdraw>, amount: f64) -> Result<()> {
        let from: &mut Account<Fundraiser> = &mut ctx.accounts.fundraiser;
        let to: &mut UncheckedAccount = &mut ctx.accounts.beneficiary;

        let _amount = amount * 1000000000 as f64;

        let fundraiser_goal = from.goal.parse::<u64>().unwrap();

        if from.raised < fundraiser_goal {
            return Err(Errors::GoalNotMet.into());
        }

        if to.key() != from.beneficiary {
            return Err(Errors::InvalidBeneficiary.into());
        }

        **from.to_account_info().try_borrow_mut_lamports()? -= _amount as u64;
        **to.try_borrow_mut_lamports()? += _amount as u64;

        let raised_subtract = amount * 100 as f64;

        from.raised -= raised_subtract as u64;

        Ok(())
    }

    #[derive(Accounts)]
    pub struct CreateUser<'info> {
        #[account(init,
            seeds = [b"fuckItWeBall!".as_ref(),
            authority.key().as_ref()],
            bump,
            payer = authority,
            space = 8 + 8 + 4 + 1048,
        )]
        pub user_account: Account<'info, User>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct UpdateUser<'info> {
        #[account(mut,
            seeds = [
                b"fuckItWeBall!".as_ref(), 
                authority.key().as_ref()], 
                bump = user_account.bump
            )]
        pub user_account: Account<'info, User>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    #[instruction(name: String)]
    pub struct CreateFundraiser<'info> {
        #[account(init, 
        seeds = [
            name.as_ref(),
            user_account.key().as_ref(),
            beneficiary.key().as_ref()], 
            bump, 
        payer = beneficiary, 
        space = 8 + 8 + 32 + 7000,
        )]
        pub fundraiser: Account<'info, Fundraiser>,
        #[account(mut)]
        pub user_account: Account<'info, User>,
        #[account(mut)]
        pub beneficiary: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct UpdateFundraiser<'info> {
        #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            fundraiser.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = fundraiser.bump)]
        pub fundraiser: Account<'info, Fundraiser>,
        #[account(mut)]
        pub user_account: Account<'info, User>,
        #[account(mut)]
        pub beneficiary: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Contribute<'info> {
        #[account(mut)]
        pub fundraiser: Account<'info, Fundraiser>,
        #[account(mut)]
        pub contributor: Signer<'info>,
        #[account(mut)]
        pub user_account: Account<'info, User>,
        /// CHECK:  This is not dangerous because we don't read or write from this account
        pub token_program: AccountInfo<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct FundraiserWithdraw<'info> {
        #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            fundraiser.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = fundraiser.bump)]
        pub fundraiser: Account<'info, Fundraiser>,
        #[account(mut)]
        pub user_account: Account<'info, User>,
        #[account(mut, constraint = beneficiary.key() == fundraiser.beneficiary )]
        pub beneficiary: UncheckedAccount<'info>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct User {
        pub authority: Pubkey,
        pub username: String,
        pub fundraisers: u8,
        pub contributions: u8,
        pub bump: u8,
        pub time: i64,
    }

    #[account]
    pub struct Fundraiser {
        pub beneficiary: Pubkey,
        pub creator: String,
        pub name: String,
        pub description: String,
        pub image_link: String,
        pub contact_link: String,
        pub website_link: String,
        pub raised: u64,
        pub goal: String,
        pub contributions: u8,
        pub bump: u8,
        pub time: i64,
        pub category: Category,
    }

    #[account]
    pub struct Contributor {
        pub contributor: Pubkey,
        pub username: String,
        pub amount: f64,
    }

    #[account]
    pub struct Beneficiary {
        pub username: String,
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
    pub enum Category {
        Individual,
        NonProfit,
        Project,
    }

    #[error_code]
    pub enum Errors {
        #[msg("Fundraiser Name is too long")]
        NameTooLong,
        #[msg("Description is too long")]
        DescriptionTooLong,
        #[msg("Invalid Authority to Update")]
        InvalidAuthority,
        #[msg("Attempting to withdraw more than Fundraiser's balance")]
        AmountTooLarge,
        #[msg("Fundraiser's goal has not been met")]
        GoalNotMet,
        #[msg("Invalid Beneficiary provided")]
        InvalidBeneficiary,
    }