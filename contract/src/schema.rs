use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Schema {
    pub name: String, // collection name equal account
    pub creator_id: AccountId,
    pub collection_id: CollectionId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SchemaJson {
    pub name: String, // collection name equal account
    pub creator_id: AccountId,
    pub collection_id: CollectionId,
}

pub trait SchemaTrait {
    //approve an account ID to transfer a token on your behalf
    fn create_schema(
        &mut self,
        creator_id: AccountId,
        name: String,
        collection_id: CollectionId
    )->SchemaJson;

    fn get_schema_by_id(
        &mut self,
        schema_id: SchemaId
    )->SchemaJson;

}

#[near_bindgen]

impl SchemaTrait for Contract {
    #[payable]
    fn create_schema(&mut self, creator_id: AccountId, name: String, collection_id: CollectionId) ->SchemaJson {
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        assert_eq!(creator_id, caller_id, "Caller is not creator_id");

        let schema_id = format!("{}", (self.schemas.len() + 1));
        // let collection = self.collections.get(&collection_id);

        assert!(
            self.schemas.get(&schema_id).is_none(),
            "Paras: duplicate schema_id"
        );


        //TODO: Ask mentor
        // assert!(Some(collection), "collection_id not exist");
        // assert!(Some(name), "schema's name is required");

        self.schemas.insert(&schema_id, &Schema {
            name: name.clone(),
            creator_id: creator_id.clone(),
            collection_id: collection_id.clone()
        });

        refund_deposit(env::storage_usage() - initial_storage_usage);

        SchemaJson{
            name,
            creator_id,
            collection_id
        }
    }

    fn get_schema_by_id(&mut self, schema_id: SchemaId) -> SchemaJson {
        let schema = self.schemas.get(&schema_id).expect("Series does not exist");

        SchemaJson{
            name: schema.name,
            creator_id: schema.creator_id,
            collection_id: schema.collection_id
        }
    }
}