use anchor_lang::prelude::*;

#[account]
pub struct Trade {
    /// The public key of the first party (initiator) of the trade
    pub party_one: Pubkey,
    /// The public key of the second party of the trade
    pub party_two: Pubkey,
    /// SOL amount to be exchanged in the transaction
    pub sol_amount: [u64;2],
    /// SPL amount to be exchanged in the transaction
    pub spl_amount: [u64;2],
    /// SPL send account of the first party (outgoing)
    pub one_send_address: Option<Pubkey>,
    /// Mint of the SPL token sent by the first party
    pub one_mint: Option<Pubkey>,
    /// SPL send account of the second party (outgoing)
    pub two_send_address: Option<Pubkey>,
    /// SPL receive account of the second party (incoming)
    pub two_receive_address: Option<Pubkey>,
    /// Mint of the SPL token sent by the second party
    pub two_mint: Option<Pubkey>,
    /// The time of creation of the trade
    pub time: i64,
    /// Whether the trade is confirmed by the second party
    pub is_confirmed: bool,
    /// The collection key (for trade time and trade fee)
    pub collection: Pubkey

}

impl Trade {
    pub const LEN: usize = 8 + 32 + 32 + 16 + 16 + 1 + 32 + 1 + 32 + 1 + 32 + 8 + 1 + 32 + 2 + 64;

    pub fn new(
        party_one: Pubkey,
        party_two: Pubkey,
        sol_amount: u64,
        spl_amount: u64,
        one_send_address: Option<Pubkey>,
        time: i64,
        collection: Pubkey,
        one_mint: Option<Pubkey>
    ) -> Self {
        Self { 
            party_one, 
            party_two, 
            sol_amount: [sol_amount,0], 
            spl_amount: [spl_amount,0], 
            one_send_address, 
            two_send_address: None,
            two_receive_address: None, 
            time, 
            is_confirmed: false,
            collection,
            one_mint,
            two_mint: None
        }
    }
}

#[derive(AnchorDeserialize,AnchorSerialize)]
pub enum TradeType {
    Sol,
    Spl,
    Both    
}