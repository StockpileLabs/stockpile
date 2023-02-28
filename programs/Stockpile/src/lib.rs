/*


█████╗█████╗█████╗
╚════╝╚════╝╚════╝



███████╗████████╗ ██████╗  ██████╗██╗  ██╗██████╗ ██╗██╗     ███████╗
██╔════╝╚══██╔══╝██╔═══██╗██╔════╝██║ ██╔╝██╔══██╗██║██║     ██╔════╝
███████╗   ██║   ██║   ██║██║     █████╔╝ ██████╔╝██║██║     █████╗
╚════██║   ██║   ██║   ██║██║     ██╔═██╗ ██╔═══╝ ██║██║     ██╔══╝
███████║   ██║   ╚██████╔╝╚██████╗██║  ██╗██║     ██║███████╗███████╗
╚══════╝   ╚═╝    ╚═════╝  ╚═════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝



█████╗█████╗█████╗
╚════╝╚════╝╚════╝

Copyright 2022 Stockpile,
www.twitter.com/GoStockpile
www.stockpile.pro

--GPL 3.0 License--
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

DISCLAIMER:
This code is currently unaudited, while reusing and duplication are allowed, please do so at your own risk.
*/

use anchor_lang::prelude::*;
//use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("CVhwkyv1yyHVwTSdTmbHSZ7HG5zNjBwxYP6gaJiX4FpA");

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod stockpile {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
        instructions::create_user(ctx, username).expect("Failed to create User.");

        Ok(())
    }

    pub fn create_individual(
        ctx: Context<CreateIndividual>,
        name: String,
        description: String,
        image_link: String,
        twitter: String,
        discord: String,
        telegram: String,
        location: String,
        goal: String,
    ) -> Result<()> {
        instructions::create_individual_fundraiser(
            ctx,
            name,
            description,
            image_link,
            twitter,
            discord,
            telegram,
            location,
            goal,
        )
        .expect("Failed to create individual fundraiser.");

        Ok(())
    }

    pub fn create_project(
        ctx: Context<CreateProject>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        twitter: String,
        discord: String,
        telegram: String,
        location: String,
        repo: String,
        goal: String,
    ) -> Result<()> {
        instructions::create_project_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            twitter,
            discord,
            telegram,
            location,
            repo,
            goal,
        )
        .expect("Failed to initialize project.");

        Ok(())
    }

    pub fn create_grant(
        ctx: Context<CreateGrant>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        twitter: String,
        discord: String,
        telegram: String,
        location: String,
        repo: String,
        goal: String,
        is_matching_eligible: bool,
    ) -> Result<()> {
        instructions::create_grant_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            twitter,
            discord,
            telegram,
            location,
            repo,
            goal,
            is_matching_eligible,
        )
        .expect("Failed to initialize grant.");

        Ok(())
    }

    pub fn create_charity(
        ctx: Context<CreateCharity>,
        name: String,
        description: String,
        image_link: String,
        website_link: String,
        twitter: String,
        discord: String,
        telegram: String,
        location: String,
        filing_link: String,
        goal: String,
    ) -> Result<()> {
        instructions::create_charity_fundraiser(
            ctx,
            name,
            description,
            image_link,
            website_link,
            twitter,
            discord,
            telegram,
            location,
            filing_link,
            goal,
        )
        .expect("Failed to create charity fundraiser.");

        Ok(())
    }

    pub fn contribute_charity(ctx: Context<ContributeCharity>, amount: u64) -> Result<()> {
        instructions::contribute_charity(ctx, amount).expect("Failed to contribute.");

        Ok(())
    }

    pub fn contribute_grant(ctx: Context<ContributeGrant>, amount: u64) -> Result<()> {
        instructions::contribute_grant(ctx, amount).expect("Failed to contribute.");

        Ok(())
    }

    pub fn contribute_individual(ctx: Context<ContributeIndividual>, amount: u64) -> Result<()> {
        instructions::contribute_individual(ctx, amount).expect("Failed to contribute.");

        Ok(())
    }

    pub fn contribute_project(ctx: Context<ContributeProject>, amount: u64) -> Result<()> {
        instructions::contribute_project(ctx, amount).expect("Failed to contribute.");

        Ok(())
    }

    pub fn update_charity(
        ctx: Context<UpdateCharity>,
        description: String,
        website_link: String,
        twitter: String,
        discord: String,
        telegram: String,
    ) -> Result<()> {
        instructions::update_charity(ctx, description, website_link, twitter, discord, telegram)
            .expect("Failed to update.");

        Ok(())
    }

    pub fn update_grant(
        ctx: Context<UpdateGrant>,
        description: String,
        website_link: String,
        twitter: String,
        discord: String,
        telegram: String,
    ) -> Result<()> {
        instructions::update_grant(ctx, description, website_link, twitter, discord, telegram)
            .expect("Failed to update.");

        Ok(())
    }

    pub fn update_individual(
        ctx: Context<UpdateIndividual>,
        description: String,
        twitter: String,
        discord: String,
        telegram: String,
    ) -> Result<()> {
        instructions::update_individual(ctx, description, twitter, discord, telegram)
            .expect("Failed to update.");

        Ok(())
    }

    pub fn update_project(
        ctx: Context<UpdateProject>,
        description: String,
        website_link: String,
        twitter: String,
        discord: String,
        telegram: String,
    ) -> Result<()> {
        instructions::update_project(ctx, description, website_link, twitter, discord, telegram)
            .expect("Failed to update.");

        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, username: String) -> Result<()> {
        instructions::update_user(ctx, username).expect("Failed to update.");

        Ok(())
    }

    pub fn withdraw_charity(ctx: Context<WithdrawCharity>, amount: u64) -> Result<()> {
        instructions::withdraw_charity(ctx, amount).expect("Failed to withdraw.");

        Ok(())
    }

    pub fn withdraw_grant(ctx: Context<WithdrawGrant>, amount: u64) -> Result<()> {
        instructions::withdraw_grant(ctx, amount).expect("Failed to withdraw.");

        Ok(())
    }

    pub fn withdraw_individual(ctx: Context<WithdrawIndividual>, amount: u64) -> Result<()> {
        instructions::withdraw_individual(ctx, amount).expect("Failed to withdraw.");

        Ok(())
    }

    pub fn withdraw_project(ctx: Context<WithdrawProject>, amount: u64) -> Result<()> {
        instructions::withdraw_project(ctx, amount).expect("Failed to withdraw.");

        Ok(())
    }
}
