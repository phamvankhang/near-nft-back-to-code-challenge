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
    name: AccountId, // collection name equal account
    creator_id: AccountId,
    created_at: u64,
    metadata: CollectionMetaData,
    collection_id: CollectionId //equal collections length
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectionData {
    name: AccountId, // collection name equal account
    creator_id: AccountId,
    created_at: u64,
    metadata: CollectionMetaData,
    collection_id: CollectionId, //equal collections length
    // schemas:
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
            collection_id: collection_id.clone()
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
}