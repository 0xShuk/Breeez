use anchor_lang::prelude::*;
use anchor_spl::{
    token::{TokenAccount,Mint},
    metadata::MetadataAccount
};

use crate::states::{Collection,Identity,Username};
use crate::{Errors, MPL_TOKEN_METADATA_ID};

#[derive(Accounts)]
#[instruction(username: String)]
pub struct CreateIdentity<'info> {
    #[account(
        init,
        payer = signer,
        space= Identity::LEN,
        seeds = [
            b"identity",
            signer.key().as_ref(),
            collection.key().as_ref()
        ],
        bump
    )]
    pub identity_details: Account<'info,Identity>,

    #[account(
        init,
        payer = signer,
        space= Username::LEN,
        seeds = [
            b"username",
            username.as_bytes(),
            collection.key().as_ref()
        ],
        bump
    )]
    pub username_details: Account<'info,Username>,

    pub collection: Box<Account<'info,Collection>>,

    #[account(
        token::authority = signer, 
        token::mint = mint, 
        constraint = token.amount == 1 @ Errors::TokenNotOne
    )]
    pub token: Account<'info,TokenAccount>,

    #[account(mint::decimals = 0)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        seeds = [
            b"metadata", 
            MPL_TOKEN_METADATA_ID.as_ref(), 
            mint.key().as_ref()
        ],
        seeds::program = MPL_TOKEN_METADATA_ID,
        bump,
        constraint = metadata.collection.as_ref().unwrap().verified @ Errors::CollectionNotVerified,
        constraint = metadata.collection.as_ref().unwrap().key ==
        collection.verified_collection_key @ Errors::CollectionNotSame

    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info,System>
}

pub fn create_identity_handler(ctx: Context<CreateIdentity>,username: String) -> Result<()> {

    require_gte!(15, username.len(), Errors::StringLengthExceeds);

    let identity_details = &mut ctx.accounts.identity_details;
    let username_details = &mut ctx.accounts.username_details;
    let signer = ctx.accounts.signer.key();

    **identity_details = Identity::new(username);
    **username_details = Username::new(signer);

    Ok(())
}
