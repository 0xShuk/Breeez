use anchor_lang::prelude::*;

use anchor_spl::metadata::MetadataAccount;

use crate::{Errors, MPL_TOKEN_METADATA_ID};

pub fn validate_metadata_account(
    mint: &Pubkey, 
    metadata: &AccountInfo, 
    verified_key: &Pubkey
) -> Result<u64> {
    
    // Verify whether the metadata account is initialized
    require_eq!(metadata.data_is_empty(), false, Errors::AccountNotInitialized);

    let metadata_seed = [
        b"metadata",
        MPL_TOKEN_METADATA_ID.as_ref(),
        mint.as_ref()
    ];

    let (metadata_pda,_) = Pubkey::find_program_address(
        &metadata_seed, 
        &MPL_TOKEN_METADATA_ID
    );

    // Assert whether the metadata key provided is equal to the one calculated using the provided mint    
    require_keys_eq!(*metadata.key,metadata_pda, Errors::WrongMetadata);

    let metadata_account: Account<MetadataAccount> = Account::try_from(metadata)?;
    
    if let Some (collection_details) = &metadata_account.collection {
        // Verifies the certified collection details
        require_eq!(collection_details.verified, true, Errors::CollectionNotVerified);
        require_keys_eq!(collection_details.key, *verified_key, Errors::CollectionNotSame);
    } else {
        return Err(Errors::CollectionNotSet.into());
    }

    // Extracts the ID of the NFT from its name in the metadata
    let nft_name = &metadata_account.data.name;
    let nft_name_trun = nft_name.replace("\x00","");
    let bytes = nft_name_trun.as_bytes();

    let mut nft_num_string = "";

    for (i,&item) in bytes.iter().enumerate() {
        if item == b'#' {
            nft_num_string = &nft_name_trun[i+1..nft_name_trun.len()];
            break;
        }
    }

    let nft_num: u64 = nft_num_string.parse().unwrap();

    Ok(nft_num)
}