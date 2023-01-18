use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    /// the address of the user
    address: Pubkey
}

#[account]
pub struct Details {
    /// the username of the user
    username: String
}

impl Identity {
    pub fn verify_identity(
        token_account: Pubkey,
        metadata_account: Pubkey
    ) -> Result<Pubkey> {
        Ok()
    }
}