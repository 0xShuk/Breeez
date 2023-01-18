use anchor_lang::prelude::*;
use crate::Errors;

#[account]
pub struct Proposal {
    /// The public key of the associated collection (32)
    pub collection: Pubkey,
    /// The text/title of the proposal (max 300)
    pub proposal: String,
    /// The vector of the options (4 + 25 * 5)
    pub options: Vec<String>,
    /// The vector of votes corresponding to the options (4 + 8 * 5)
    pub votes: Vec<u64>,
    /// The record of the voters who voted (4 + (NFT count / 8) + 1)
    pub voters: Vec<u8>,
    /// Whether the proposal is active or closed (1)
    pub is_active: bool,
    /// Whether the proposal is passed or not (1)
    pub is_passed: bool,
    /// The time of the proposal creation (8)
    pub time: i64,
    /// The creator of the proposal (32)
    pub creator: Pubkey
}

impl Proposal {
    pub fn len(
        nft_count: u64,
        title_size: usize,
        option_count: usize
    ) -> usize {
        8 + 32 + title_size + 4 + (25 * option_count) +
        4 + (8 * option_count) + 4 + 1 + nft_count as usize + 
        2 + 8 + 32
    }

    pub fn new(
        collection: Pubkey, 
        proposal: String,
        votes: Vec<u64>,
        voters: Vec<u8>,
        options: Vec<String>,
        creator: Pubkey
    ) -> Self {
        let clock = Clock::get().unwrap();
        let time = clock.unix_timestamp;
        Self { 
            collection,
            proposal,
            votes,
            voters,
            options,
            is_active: false,
            is_passed: false,
            time,
            creator
        }
    }

    pub fn increase_vote(&mut self, choice: u8) {
        self.votes[choice as usize] += 1;
    }

    pub fn record_voter(&mut self, nft_index: usize, nft_pos: usize) -> Result<()> {
        let current_votes = self.voters[nft_index];
        let mut current_votes_string = format!("{:08b}",current_votes);

        require_eq!(
            &current_votes_string[nft_pos .. nft_pos + 1],
            "0",
            Errors::AlreadyVoted
        );

        current_votes_string.replace_range(nft_pos .. nft_pos+1, "1");

        self.voters[nft_index] = u8::from_str_radix(&current_votes_string, 2).unwrap();

        Ok(())
    }

}