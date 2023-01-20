use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use crate::{states::Collection,Errors};

#[derive(Accounts)]
pub struct EditStake<'info> {
    #[account(
        mut,
        seeds = [b"collection", verified_collection.key().as_ref()],
        bump
    )]
    pub collection_details: Account<'info, Collection>,

    pub owner: Signer<'info>,
    
    #[account(
        constraint = verified_collection.collection == None @ Errors::NotCollectionNft,
        constraint = verified_collection.update_authority == owner.key()
        @ Errors::NotUpdateAuthority
    )]
    pub verified_collection: Account<'info, MetadataAccount>,

}

pub fn edit_stake_handler(ctx: Context<EditStake>, emission: u64) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;

    require_gt!(emission, 0, Errors::ZeroValue);

    collection_details.emission = emission;
    
    Ok(())
}