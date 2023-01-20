use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    /// The username of the holder (4 + 15)
    pub username: String
}

#[account]
pub struct Username {
    /// The public key of the holder (32)
    pub key: Pubkey
}

impl Identity {
    pub const LEN: usize = 8 + 4 + 15;
    
    pub fn new(username: String) -> Self {
        Self {
            username
        }
    }
}

impl Username {
    pub const LEN: usize = 8 + 32;
    
    pub fn new(key: Pubkey) -> Self {
        Self {
            key
        }
    }
}