use anchor_lang::prelude::*;

#[account]
pub struct Stake {
    /// The pubkey of the owner of the NFT (32)
    pub owner: Pubkey,
    /// The time of the staking (8)
    pub time: i64,
    /// The token account from which the NFT is sent (32)
    pub nft_send_address: Pubkey,
    /// The pubkey of the collection details account
    pub collection: Pubkey
}

impl Stake {
    pub const LEN: usize = 8 + 32 + 8 + 32 + 32; 

    pub fn new(
        owner: Pubkey,
        nft_send_address: Pubkey,
        collection: Pubkey
    ) -> Self {
        let clock = Clock::get().unwrap();
        let time = clock.unix_timestamp;

        Self { 
            owner, 
            time,
            nft_send_address,
            collection
        }
    }
}