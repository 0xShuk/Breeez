use anchor_lang::prelude::*;

use crate::states::{Collection,Proposal};
use crate::Errors;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {

    pub collection: Account<'info, Collection>,

    #[account(
        mut,
        has_one = collection
    )]
    pub proposal_details: Account<'info, Proposal>,

    pub signer: Signer<'info>,
}

pub fn execute_proposal_handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    let collection_details = &ctx.accounts.collection;
    let proposal_details = &mut ctx.accounts.proposal_details;

    let clock = Clock::get()?;
    let current_unix = clock.unix_timestamp;

    let vote_duration = collection_details.vote_duration;
    let quorum = collection_details.quorum;

    let proposal_time = proposal_details.time;

    require_eq!(proposal_details.is_active, true, Errors::ProposalAlreadyExecuted);

    require_gt!(current_unix, proposal_time + vote_duration, Errors::VotingIsActive);

    let votes = &proposal_details.votes;
    let sum_of_votes: u64 = votes.iter().sum();

    if sum_of_votes >= quorum {
        proposal_details.is_passed = true;
    }

    proposal_details.is_active = false;

    Ok(())
}