use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey, pubkey::Pubkey};
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;
use states::TradeType;
declare_id!("FyAhN3NB5CkiW3tqhPKU7e3UPQUP5DuBD4oMUrww6Ah2");

#[constant]
pub const MPL_TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

#[program]
pub mod sol {
    use super::*;

    /// * Creates account which holds the info about the collection
    /// * only validates the Collection field i.e. it isn't a regular NFT
    /// * doesn't validate the CollectionDetails field (since many collections don't set this field)
    pub fn create_collection(ctx: Context<CreateCollection>,treasury: Pubkey) -> Result<()> {
        instructions::create_collection_handler(ctx, treasury)
    }

    pub fn add_voting(ctx: Context<AddVoting>, count: u64, duration: i64, quorum: u64) -> Result<()> {
        instructions::add_voting_handler(ctx, count, duration, quorum)
    }

    pub fn edit_voting(ctx: Context<EditVoting>, new_num: u64, edit_type: VotingEditType) -> Result<()> {
        instructions::edit_voting_handler(ctx, new_num, edit_type)
    }

    pub fn add_trade(ctx: Context<AddTrade>, trade_fee: u64, duration: i64) -> Result<()> {
        instructions::add_trade_handler(ctx, trade_fee, duration)
    }

    pub fn edit_trade(ctx: Context<EditTrade>, new_num: u64, edit_type: TradeEditType) -> Result<()> {
        instructions::edit_trade_handler(ctx, new_num, edit_type)
    }

    pub fn create_trade(
        ctx: Context<CreateTrade>,
        sol_amount: u64,
        spl_amount: u64,
        trade_type: TradeType
    ) -> Result<()> {
        instructions::create_trade_handler(ctx, sol_amount, spl_amount, trade_type)
    }

    pub fn cancel_trade(ctx: Context<CancelTrade>) -> Result<()> {
        instructions::cancel_trade_handler(ctx)
    }

    pub fn accept_trade(
        ctx: Context<AcceptTrade>,
        sol_amount: u64,
        spl_amount: u64,
        trade_type: TradeType
    ) -> Result<()> {
        instructions::accept_trade_handler(ctx, sol_amount, spl_amount, trade_type)
    }

    pub fn execute_trade(ctx: Context<ExecuteTrade>) -> Result<()> {
        instructions::execute_trade_handler(ctx)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal: String,
        options: Vec<String>
    ) -> Result<()> {
        instructions::create_proposal_handler(ctx, proposal, options)
    }

    pub fn give_vote(ctx: Context<GiveVote>,choice: u8) -> Result<()> {
        instructions::give_vote_handler(ctx, choice)
    }

}

#[error_code]
pub enum Errors {
    #[msg("Not a Collection NFT")]
    NotCollectionNft,

    #[msg("The signer is not the update authority")]
    NotUpdateAuthority,

    #[msg("The time is expired")]
    TradeTimeExpired,

    #[msg("The time is not expired")]
    TradeTimeNotExpired,

    #[msg("The module is already added")]
    ModuleAlreadyAdded,

    #[msg("The module is not active")]
    ModuleNotActive,

    #[msg("The holder must be holding exactly 1 token")]
    TokenNotOne,

    #[msg("Validation failed: Collection key is not verified")]
    CollectionNotVerified,

    #[msg("Validation failed: Collection key doesn't match")]
    CollectionNotSame,

    #[msg("Trade amount can't be zero")]
    TokenAmountZero,

    #[msg("The inactive token must be zero")]
    AmountNotZero,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("SPL Token Account is not provided")]
    AccountNotProvided,

    #[msg("the Escrow Account is not required in this trade")]
    AccountNotRequired,

    #[msg("The signer is not a party to the trade")]
    NotTradeParty, 

    #[msg("The trade is already accepted by the party two")]
    TradeAlreadyAccepted,

    #[msg("The trade is not yet accepted by the party two")]
    TradeNotAccepted,

    #[msg("The mint account doesn't exist in the trade")]
    MintNotExist,

    #[msg("The account details doesn't match with the trade")]
    IncorrectTokenAccount,

    #[msg("The number of options must be between 2 to 5")]
    IncorrectOptionCount,

    #[msg("The string length exceeds the maximum permissible")]
    StringLengthExceeds,

    #[msg("The string length is zero")]
    BlankStringFound,

    #[msg("The voting is closed for this proposal")]
    VotingIsClosed,

    #[msg("The voting is still active for this proposal")]
    VotingIsActive,

    #[msg("The provided account is not initialized")]
    AccountNotInitialized,

    #[msg("The Collection field is not set in the metadata")]
    CollectionNotSet,

    #[msg("The NFT used for the voting has already voted")]
    AlreadyVoted,

    #[msg("The selected option doesn't exist")]
    OptionNotExists,

    #[msg("The value cannot be zero")]
    ZeroValue,

    #[msg("The quorum cannot exceed the NFT count")]
    InvalidQuorum,

    #[msg("The proposal is already executed")]
    ProposalAlreadyExecuted,

    #[msg("The metadata PDA doesn't match with the token mint")]
    WrongMetadata
}
