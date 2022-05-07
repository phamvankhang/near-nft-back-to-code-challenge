use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionMetaData {
    name: AccountId, // collection name equal account
    website: String,
    telegram: String,
    discord: String,
    market_fee: u32
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionJson {
    pub name: AccountId, // collection name equal account
    pub creator_id: AccountId,
    pub created_at: u64,
    pub metadata: CollectionMetaData,
    pub collection_id: CollectionId //equal collections length
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectionData {
    pub name: AccountId, // collection name equal account
    pub creator_id: AccountId,
    pub created_at: u64,
    pub metadata: CollectionMetaData,
    pub collection_id: CollectionId, //equal collections length
    pub schemas: UnorderedMap<SchemaId, Schema>,
    pub templates: UnorderedMap<TemplateId, Template>,
    pub tokens: UnorderedMap<TokenId, Token>,
}


pub trait Collection {
    //approve an account ID to transfer a token on your behalf

    fn create_collection(
        &mut self,
        name: AccountId,
        creator_id: AccountId,
        collection_metadata: CollectionMetaData
    )->CollectionId;

    fn get_collection_by_id(
        &mut self,
        collection_id: CollectionId
    )->CollectionJson;

    fn get_collections(
        &self,
        offset: Option<U128>,
        limit: Option<u64>
    )->Vec<CollectionJson>;
}

#[near_bindgen]
impl Collection for Contract {

    #[payable]
    fn create_collection(&mut self, name: AccountId, creator_id: AccountId, collection_metadata: CollectionMetaData) ->CollectionId {

        let collection_id = format!("{}", (self.collections.len() + 1));
        let caller_id = env::predecessor_account_id();

        assert_eq!(creator_id, caller_id, "Paras: Caller is not creator_id");

        let metadata = CollectionMetaData{
            name: name.clone(),
            website: collection_metadata.website,
            discord: collection_metadata.discord,
            market_fee: collection_metadata.market_fee,
            telegram: collection_metadata.telegram,
        };

        let collection = CollectionData{
            name,
            creator_id: caller_id,
            created_at: env::block_timestamp(),
            metadata,
            collection_id: collection_id.clone(),
            schemas: UnorderedMap::new(
                StorageKey::CollectionSchemas
                .try_to_vec()
                .unwrap(),
            ),
            templates: UnorderedMap::new(
                StorageKey::CollectionTemplates
                .try_to_vec()
                .unwrap(),
            ),
            // templates:
            tokens: UnorderedMap::new(StorageKey::CollectionTokens
                    .try_to_vec()
                    .unwrap(),
            ),
        };
        assert!(
            self.collections.get(&collection_id).is_none(),
            "Error: duplicate CollectionId"
        );

        self.collections.insert(&collection_id, &collection);

        collection_id
    }

    fn get_collection_by_id(&mut self, collection_id: CollectionId) -> CollectionJson {
        let collection = self.collections.get(&collection_id).expect("Collection does not exist");

        CollectionJson{
            name: collection.name,
            creator_id: collection.creator_id,
            created_at: collection.created_at,
            metadata: collection.metadata,
            collection_id: collection.collection_id
        }
    }

    fn get_collections(&self, offset: Option<U128>, limit: Option<u64>) ->Vec<CollectionJson> {
        let start_index: u128 = offset.map(From::from).unwrap_or_default();
        assert!(
            (self.collections.len() as u128) > start_index,
            "Out of bounds, please use a smaller offset."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        self.collections
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(_collection_id, collection)| CollectionJson{
                name: collection.name,
                creator_id: collection.creator_id,
                created_at: collection.created_at,
                metadata: collection.metadata,
                collection_id: collection.collection_id,
            })
            .collect()
    }
}