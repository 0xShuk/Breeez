use anchor_lang::prelude::*;
use anchor_spl::{
    token::{TokenAccount,Mint},
    metadata::MetadataAccount
};

use crate::states::{Collection,Proposal};
use crate::{Errors, MPL_TOKEN_METADATA_ID};

#[derive(Accounts)]
#[instruction(proposal: String, options: Vec<String>)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = signer,
        space= Proposal::len(
            collection_details.count / 8,
            proposal.len(),
            options.len()
        ),
    )]
    pub proposal_details: Account<'info,Proposal>,

    pub collection_details: Box<Account<'info,Collection>>,

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
        collection_details.verified_collection_key @ Errors::CollectionNotSame

    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info,System>
}

pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    proposal: String,
    options: Vec<String>
) -> Result<()> {

    let proposal_details = &mut ctx.accounts.proposal_details;
    let collection_details = &ctx.accounts.collection_details;
    let creator = ctx.accounts.signer.key();

    let is_voting_active = collection_details.is_voting;
    require_eq!(is_voting_active,true, Errors::ModuleNotActive);

    require_gt!(options.len(), 1, Errors::IncorrectOptionCount);
    require_gt!(6, options.len(), Errors::IncorrectOptionCount);

    let nft_count = (collection_details.count / 8) + 1;
    let proposal_length = proposal.len();

    let mut votes = Vec::new();
    let mut voters = Vec::new();

    require_gte!(300, proposal_length, Errors::StringLengthExceeds);
    require_gt!(proposal_length, 0, Errors::BlankStringFound);

    for i in 0..options.len() {
        if options[i].len() > 25 {
            return Err(Errors::StringLengthExceeds.into());
        } else if options[i].len() < 1 {
            return Err(Errors::BlankStringFound.into());
        }
        votes.push(0);
    }

    for _i in 0..nft_count {
        voters.push(0);
    }

    **proposal_details = Proposal::new(
        collection_details.key(), 
        proposal, 
        votes, 
        voters, 
        options,
        creator
    );

    Ok(())
}
