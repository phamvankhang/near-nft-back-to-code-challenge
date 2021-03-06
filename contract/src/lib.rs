use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
    // serde_json::json,
    // env::is_valid_account_id,
    // env::panic_str,
    Timestamp
};

// use chrono::{
    // DateTime,
    // TimeZone,
    // NaiveDateTime,
    // Utc
// };

//custom type
pub type TimestampSec = u32;
pub type TemplateId = String;
pub type CollectionId = String;
pub type SchemaId = String;
pub const TOKEN_DELIMETER: char = ':';

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod internal;
mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod events;

//CUSTOM
pub use crate::collection::*;
pub use crate::schema::*;
pub use crate::template::*;
mod collection;
mod schema;
mod template;

/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

// #[derive(BorshSerialize, BorshStorageKey)]
/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,

    //CUSTOM
    Collections,
    //nested
    CollectionTemplates,
    CollectionSchemas,
    CollectionTokens,
    TemplateAttribute,
    TokensByTemplateInner,
    TokensByTemplateId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //CUSTOM
    pub collections: UnorderedMap<CollectionId, CollectionData>,
    // pub schemas: UnorderedMap<SchemaId, Schema>,
    // pub templates: UnorderedMap<TemplateId, Template>,
    pub transaction_fee: u16,

    pub tokens: UnorderedMap<TokenId, Token>,//find any token by id
}


#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "NFT Tutorial Contract".to_string(),
                symbol: "XBOX".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized.
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            collections: UnorderedMap::new(StorageKey::Collections.try_to_vec().unwrap()),
            tokens: UnorderedMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            // templates: UnorderedMap::new(StorageKey::Template.try_to_vec().unwrap()),
            // tokens_by_template_id: UnorderedMap::new(StorageKey::TokensByTemplateId.try_to_vec().unwrap()),
            transaction_fee: 2
        };

        //return the Contract object
        this
    }

    pub fn to_sec(timestamp: Timestamp) -> TimestampSec {
        (timestamp / 10u64.pow(9)) as u32
    }
}

