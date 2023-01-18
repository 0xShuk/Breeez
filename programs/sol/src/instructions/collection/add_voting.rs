use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use crate::states::Collection;

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
        constraint = verified_collection.collection == None,
        constraint = verified_collection.update_authority == owner.key()
    )]
    pub verified_collection: Account<'info, MetadataAccount>,

}

pub fn add_voting_handler(ctx: Context<AddVoting>, count: u64, duration: i64, quorum: u64) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;

    collection_details.is_voting = true;
    collection_details.quorum = quorum;
    collection_details.count = count;
    collection_details.vote_duration = duration;
    Ok(())
}
