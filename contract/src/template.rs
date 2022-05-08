use crate::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TemplateJson {
    pub name: String,
    // collection name equal account
    pub creator_id: AccountId,
    // pub created_at: TimestampSec,
    pub token_metadata: TokenMetadata,
    pub schema_id: SchemaId,
    pub collection_id: CollectionId,
    pub max_supply: u64,
    pub is_mintable: bool,
    pub transferable: bool,
    pub burnable: bool,
    pub issued: usize,
    pub nft_type: String,//NFT, BOX, Whitelist
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Template {
    pub name: String,
    // collection name equal account
    pub creator_id: AccountId,
    // pub created_at: TimestampSec,
    pub token_metadata: TokenMetadata,
    pub schema_id: SchemaId,
    pub collection_id: CollectionId,
    pub max_supply: u64,
    pub is_mintable: bool,
    pub transferable: bool,
    pub burnable: bool,
    pub issued: usize,
    pub nft_type: String,//NFT or BOX
}

pub trait TemplateTrait {
    //approve an account ID to transfer a token on your behalf
    fn create_template(
        &mut self,
        creator_id: AccountId,
        name: String,
        nft_type: String,
        schema_id: SchemaId,
        collection_id: CollectionId,
        token_metadata: TokenMetadata,
    ) -> TemplateJson;

    // fn get_templates_by_creator_id(
    //     &self,
    //     creator_id: AccountId,
    // ) -> Vec<TemplateJson>;
}

#[near_bindgen]
impl TemplateTrait for Contract {
    #[payable]
    fn create_template(&mut self,
                       creator_id: AccountId,
                       name: String,
                       nft_type: String,
                       schema_id: SchemaId,
                       collection_id: CollectionId,
                       token_metadata: TokenMetadata,
    ) -> TemplateJson {
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        assert_eq!(creator_id, caller_id, "Caller is not creator_id");

        let mut collection = self.collections.get(&collection_id).expect("collection not exist");

        let template_id = format!("{}", (collection.templates.len() + 1));

        assert!(
            collection.templates.get(&template_id).is_none(),
            "duplicate template_id"
        );

        let title = token_metadata.title.clone();
        assert!(title.is_some(), "Paras: token_metadata.title is required");

        let template = Template {
            name: name.clone(),
            token_metadata,
            schema_id: schema_id.clone(),
            collection_id: collection_id.clone(),
            creator_id: caller_id,
            transferable: true,
            burnable: true,
            is_mintable: true,
            nft_type,
            // created_at: to_sec(env::block_timestamp()),
            max_supply: 0,
            issued: 0,
        };


        collection.templates.insert(&template_id, &template);

        refund_deposit(env::storage_usage() - initial_storage_usage);

        TemplateJson {
            name: template.name,
            creator_id: template.creator_id,
            token_metadata: template.token_metadata,
            schema_id: template.schema_id,
            collection_id: template.collection_id,
            max_supply: 0,
            nft_type: template.nft_type,
            is_mintable: false,
            transferable: false,
            burnable: false,
            issued: 0,
        }
    }

    // TODO: refactor collection to ez get data
    // fn get_templates_by_creator_id(&self, creator_id: AccountId) -> Vec<TemplateJson> {
    //     let collection = self.collections
    //         .iter()
    //         .filter_map(|(key, value)| {
    //             if value.creator_id == creator_id {
    //                 Some((key, value))
    //             } else {
    //                 None
    //             }
    //         })
    //         .map(|(_collection_id, col)| CollectionData {
    //             name: col.name,
    //             creator_id: col.creator_id,
    //             created_at: col.created_at,
    //             metadata: col.metadata,
    //             collection_id: col.collection_id,
    //             schemas: col.schemas,
    //             templates: col.templates,
    //             tokens: col.tokens
    //         })
    //         .collect();
    //
    //     //TODO
    //     println!("{}", &collection);
    //     &collection.templates.iter().collect()
    // }
}