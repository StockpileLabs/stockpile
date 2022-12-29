use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock;
use anchor_lang::system_program;

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("4BjCifBQ6teHWgf3WmjJmR8n75oF1oWbdNFWQTvsz7o3");

#[program]
pub mod stockpile {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
        create_user(ctx, username);
    }

    pub fn create_individual(
        ctx: Context<CreateIndividual>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        contact_link: String,
        goal: String,
    ) -> Result<()> {
        create_individual_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            contact_link,
            goal,
        );
    }

    pub fn create_project(
        ctx: Context<CreateProject>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        contact_link: String,
        goal: String,
    ) -> Result<()> {
        create_project_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            contact_link,
            goal,
        );
    }

    pub fn create_grant(
        ctx: Context<CreateGrant>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        contact_link: String,
        repo: String,
        goal: String,
    ) -> Result<()> {
        create_grant_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            contact_link,
            repo,
            goal,
        );
    }

    pub fn create_charity(
        ctx: Context<CreateCharity>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        contact_link: String,
        filing_link: String,
        goal: String,
    ) -> Result<()> {
        create_charity_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            contact_link,
            filing_link,
            goal,
        );
    }

    pub fn contribute_charity(ctx: Context<ContributeCharity>, amount: u64) -> Result<()> {
        contribute_charity(ctx, amount);
    }

    pub fn contribute_grant(ctx: Context<ContributeGrant>, amount: u64) -> Result<()> {
        contribute_grant(ctx, amount);
    }

    pub fn contribute_individual(ctx: Context<ContributeIndividual>, amount: u64) -> Result<()> {
        contribute_individual(ctx, amount);
    }

    pub fn contribute_project(ctx: Context<ContributeProject>, amount: u64) -> Result<()> {
        contribute_project(ctx, amount);
    }

    pub fn update_charity(
        ctx: Context<UpdateCharity>,
        description: String,
        website_link: String,
        contact_link: String,
    ) -> Result<()> {
        update_charity(ctx, description, website_link, contact_link);
    }

    pub fn update_grant(
        ctx: Context<UpdateGrant>,
        description: String,
        website_link: String,
        contact_link: String,
    ) -> Result<()> {
        update_grant(ctx, description, website_link, contact_link);
    }

    pub fn update_individual(
        ctx: Context<UpdateIndividual>,
        description: String,
        website_link: String,
        contact_link: String,
    ) -> Result<()> {
        update_individual(ctx, description, website_link, contact_link);
    }

    pub fn update_project(
        ctx: Context<UpdateProject>,
        description: String,
        website_link: String,
        contact_link: String,
    ) -> Result<()> {
        update_project(ctx, description, website_link, contact_link);
    }

    pub fn update_user(ctx: Context<UpdateUser>, username: String) -> Result<()> {
        update_user(ctx, username);
    }

    pub fn withdraw_charity(ctx: Context<WithdrawCharity>, amount: u64) -> Result<()> {
        withdraw_charity(ctx, amount);
    }

    pub fn withdraw_grant(ctx: Context<WithdrawGrant>, amount: u64) -> Result<()> {
        withdraw_grant(ctx, amount)
    }

    pub fn withdraw_individual(ctx: Context<WithdrawIndividual>, amount: u64) -> Result<()> {
        withdraw_individual(ctx, amount);
    }

    pub fn withdraw_project(ctx: Context<WithdrawProject>, amount: u64) -> Result<()> {
        withdraw_project(ctx, amount);
    }
}
