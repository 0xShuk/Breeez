use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use crate::{states::Collection,Errors};

#[derive(Accounts)]
pub struct EditVoting<'info> {
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

pub fn edit_voting_handler(ctx: Context<EditVoting>, new_num: u64, edit_type: VotingEditType) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;

    require_gt!(new_num, 0, Errors::ZeroValue);

    match edit_type {
        VotingEditType::Count => {
            collection_details.count = new_num;
        },
        VotingEditType::Duration => {
            collection_details.vote_duration = new_num as i64;
        },
        VotingEditType::Quorum => {
            collection_details.quorum = new_num;
        }
    }
    Ok(())
}

#[derive(AnchorSerialize,AnchorDeserialize)]
pub enum VotingEditType {
    Count,
    Duration,
    Quorum
}