use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount,token::Mint};
use crate::{states::Collection,Errors,MPL_TOKEN_METADATA_ID};

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(
        init,
        payer = owner,
        space = Collection::LEN,
        seeds = [b"collection", verified_collection.key().as_ref()],
        bump
    )]
    pub collection_details: Account<'info, Collection>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [
            b"metadata", 
            MPL_TOKEN_METADATA_ID.as_ref(), 
            mint.key().as_ref()
        ],
        seeds::program = MPL_TOKEN_METADATA_ID,
        bump,
        constraint = verified_collection.collection == None @Errors::NotCollectionNft,
        constraint = verified_collection.update_authority == owner.key()
        @ Errors::NotUpdateAuthority
    )]
    pub verified_collection: Account<'info, MetadataAccount>,

    pub mint: Account<'info, Mint>,

    system_program: Program<'info, System>
}

pub fn create_collection_handler(ctx: Context<CreateCollection>,treasury: Pubkey) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;
    let verified_collection_key = ctx.accounts.mint.key();

    **collection_details = Collection::new(
        verified_collection_key, 
        treasury);
    Ok(())
}