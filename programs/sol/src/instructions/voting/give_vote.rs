use anchor_lang::prelude::*;

use crate::states::{Collection,Proposal};
use crate::{Errors,utils};

#[derive(Accounts)]
pub struct GiveVote<'info> {

    pub collection: Account<'info, Collection>,

    #[account(
        mut,
        has_one = collection
    )]
    pub proposal_details: Account<'info, Proposal>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn give_vote_handler(ctx: Context<GiveVote>,choice: u8) -> Result<()> {
    let signer = ctx.accounts.signer.key;
    let collection_details = & ctx.accounts.collection;
    let proposal_details = &mut ctx.accounts.proposal_details;

    let clock = Clock::get()?;
    let current_unix = clock.unix_timestamp;

    let vote_duration = collection_details.vote_duration;
    let proposal_time = proposal_details.time;

    require_gte!(proposal_time + vote_duration, current_unix, Errors::VotingIsClosed);
    require_gt!(proposal_details.options.len(), choice as usize, Errors::OptionNotExists);

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    require_gt!(remaining_accounts.len(),1, Errors::AccountNotProvided);

    for _i in 0 .. remaining_accounts.len() / 2 {
        
        let token_account = next_account_info(remaining_accounts)?;
        let mint = utils::validate_token_account(token_account, signer)?;
        let metadata = next_account_info(remaining_accounts)?;

        let nft_num = utils::validate_metadata_account(
            &mint, 
            metadata, 
            &collection_details.verified_collection_key
        )?;

        let nft_index: usize = (nft_num / 8).try_into().unwrap();
        let nft_pos: usize = (nft_num % 8).try_into().unwrap();

        proposal_details.increase_vote(choice);
        proposal_details.record_voter(nft_index, nft_pos)?;
    }

    Ok(())
}