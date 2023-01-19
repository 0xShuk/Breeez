use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use crate::{states::Collection,Errors};

#[derive(Accounts)]
pub struct AddVoting<'info> {
    #[account(
        mut,
        seeds = [b"collection", verified_collection.key().as_ref()],
        bump
    )]
    pub collection_details: Account<'info, Collection>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        constraint = verified_collection.collection == None @ Errors::NotCollectionNft,
        constraint = verified_collection.update_authority == owner.key()
        @ Errors::NotUpdateAuthority
    )]
    pub verified_collection: Account<'info, MetadataAccount>,

}

pub fn add_voting_handler(ctx: Context<AddVoting>, count: u64, duration: i64, quorum: u64) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;

    require_eq!(collection_details.is_voting, false, Errors::ModuleAlreadyAdded);

    require_gt!(count, 0, Errors::ZeroValue);
    require_gt!(duration, 0, Errors::ZeroValue);
    require_gt!(quorum, 0, Errors::ZeroValue);

    require_gte!(count, quorum, Errors::InvalidQuorum);

    collection_details.is_voting = true;
    collection_details.quorum = quorum;
    collection_details.count = count;
    collection_details.vote_duration = duration;
    Ok(())
}
