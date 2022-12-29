use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateCharity<'info> {
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
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_charity(
    ctx: Context<UpdateCharity>,
    description: String,
    website_link: String,
    contact_link: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let charity = &mut ctx.accounts.charity;

    if beneficiary.key() != charity.beneficiary {
        return Err(Errors::InvalidBeneficiary.into());
    } else {
        if description.len() > 0 {
            charity.description = description;
        }

        if contact_link.len() > 0 {
            charity.contact_link = contact_link;
        }

        if website_link.len() > 0 {
            charity.website_link = website_link;
        }
    }

    Ok(())
}