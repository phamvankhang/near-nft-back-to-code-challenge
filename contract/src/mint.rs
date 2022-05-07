use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        template_id: TemplateId,
        // receiver_id: AccountId
    ) -> TokenId {

        // TODO
        template_id
        // let initial_storage_usage = env::storage_usage();
        //
        // let template = self.templates.get(&template_id).expect("Template is not exist");
        // assert_eq!(env::predecessor_account_id(), template.creator_id, "not creator");
        // let token_id: TokenId = self._nft_mint_template(template_id, receiver_id);
        //
        // refund_deposit(env::storage_usage() - initial_storage_usage);
        //
        // token_id
        //
    }

    fn _nft_mint_template(
        &mut self,
        template_id: TemplateId,
        // receiver_id: AccountId
    ) -> TokenId {
        template_id
        //
        // let mut template = self.templates.get(&template_id).expect("Token template not exist");
        //
        // let tokens_by_template = self.tokens_by_template_id.get(&template_id);
        // let num_tokens = tokens_by_template.len();
        // let max_copies = template.max_supply.unwrap_or(u64::MAX);
        // assert!(num_tokens < max_copies, "Series supply maxed");
        //
        // if (num_tokens + 1) >= max_copies {
        //     template.is_mintable = false;
        // }
        //
        //
        // let token_id = format!("{}{}{}", &template_id, TOKEN_DELIMETER, num_tokens + 1);
        //
        // return token_id;// TODO implement template
        //
        // template.tokens.insert(&token_id);
        // self.token_series_by_id.insert(&template_id, &template);
        //
        // // you can add custom metadata to each token here
        // let metadata = Some(TokenMetadata {
        //     title: None,          // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
        //     description: None,    // free-form description
        //     media: None, // URL to associated media, preferably to decentralized, content-addressed storage
        //     media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
        //     copies: None, // number of copies of this set of metadata in existence when token was minted.
        //     issued_at: Some(env::block_timestamp().to_string()), // ISO 8601 datetime when token was issued or minted
        //     expires_at: None, // ISO 8601 datetime when token expires
        //     starts_at: None, // ISO 8601 datetime when token starts being valid
        //     updated_at: None, // ISO 8601 datetime when token was last updated
        //     extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
        //     reference: None, // URL to an off-chain JSON file with more info.
        //     reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        // });
        //
        // //let token = self.tokens.mint(token_id, receiver_id, metadata);
        // // From : https://github.com/near/near-sdk-rs/blob/master/near-contract-standards/src/non_fungible_token/core/core_impl.rs#L359
        // // This allows lazy minting
        //
        // let owner_id: AccountId = receiver_id;
        // self.tokens.owner_by_id.insert(&token_id, &owner_id);
        //
        // self.tokens
        //     .token_metadata_by_id
        //     .as_mut()
        //     .and_then(|by_id| by_id.insert(&token_id, &metadata.as_ref().unwrap()));
        //
        // if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
        //     let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
        //         UnorderedSet::new(StorageKey::TokensPerOwner {
        //             account_hash: env::sha256(&owner_id.as_bytes()),
        //         })
        //     });
        //     token_ids.insert(&token_id);
        //     tokens_per_owner.insert(&owner_id, &token_ids);
        // }
        //
        //
        // token_id
    }

}