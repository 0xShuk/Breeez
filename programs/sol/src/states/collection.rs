use anchor_lang::prelude::*;

#[account]
pub struct Collection {
    /// The verified key of the collection
    pub verified_collection_key: Pubkey,
    /// The treasury address (to receive trade fee)
    pub treasury_address: Pubkey,
    /// The mint address of the token associated with the collection
    pub token_mint: Option<Pubkey>,
    /// Is Staking Module active
    pub is_staking: bool,
    /// Is Voting Module active
    pub is_voting: bool,
    /// Is Trade module active
    pub is_trade: bool,
    /// Is Community module active
    pub is_community: bool,
    /// The number of NFTs in the collection
    pub count: u64,
    /// The duration for the voting proposal (in seconds)
    pub vote_duration: i64,
    /// The minimum votes required to consider proposal as passed
    pub quorum: u64,
    /// The duration for the trade (in seconds)
    pub trade_duration: i64,
    /// The fees for the trade (to be sent to treasury address - in Lamports)
    pub trade_fees: u64
}

impl Collection {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 32 + 1 + 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8; 

    pub fn new(
        verified_collection_key: Pubkey,
        treasury_address: Pubkey
    ) -> Self {
        Self { 
            verified_collection_key, 
            treasury_address, 
            token_mint: None, 
            is_staking: false, 
            is_voting: false, 
            is_trade: false, 
            is_community: false,
            count: 0,
            vote_duration: 0,
            quorum: 0,
            trade_duration: 0,
            trade_fees: 0
        }
    }
}