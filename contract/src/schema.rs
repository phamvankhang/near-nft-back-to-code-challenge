use crate::*;
use std::any::type_name;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Schema {
    pub name: String, // collection name equal account
    pub creator_id: AccountId,
    pub collection_id: CollectionId,
    pub schema_id: SchemaId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SchemaJson {
    pub name: String, // collection name equal account
    pub creator_id: AccountId,
    pub collection_id: CollectionId,
    pub schema_id: SchemaId,
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
        &self,
        schema_id: SchemaId,
        collection_id: CollectionId
    )->SchemaJson;

    fn get_schema_by_collection_id(
        &self,
        collection_id: CollectionId,
        offset: Option<U128>,
        limit: Option<u64>
    )->Vec<SchemaJson>;

}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[near_bindgen]

impl SchemaTrait for Contract {
    #[payable]
    fn create_schema(&mut self, creator_id: AccountId, name: String, collection_id: CollectionId) ->SchemaJson {
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        assert_eq!(creator_id, caller_id, "Caller is not creator_id");

        let mut collection = self.collections.get(&collection_id).expect("collection not exist");

        println!("typeof collection.schemas {}", type_of(&collection.schemas));
        // if type_of(&collection.schemas) != "near_sdk::collections::UnorderedMap".to_string() {
        //
        //     // create schemas Map if not exist
        //     collection.schemas = UnorderedMap::new(
        //         StorageKey::Schema {
        //             collection: collection_id.clone(),
        //         }
        //             .try_to_vec()
        //             .unwrap(),
        //     )
        // }

        let schema_id = format!("{}", (collection.schemas.len() + 1));
        // let collection = self.collections.get(&collection_id);

        assert!(
            collection.schemas.get(&schema_id).is_none(),
            "Paras: duplicate schema_id"
        );


        //TODO: Ask mentor
        // assert!(Some(collection), "collection_id not exist");
        // assert!(Some(name), "schema's name is required");

        collection.schemas.insert(&schema_id, &Schema {
            name: name.clone(),
            creator_id: creator_id.clone(),
            collection_id: collection_id.clone(),
            schema_id: schema_id.clone(),
        });

        refund_deposit(env::storage_usage() - initial_storage_usage);


        SchemaJson{
            name,
            creator_id,
            collection_id,
            schema_id
        }
    }

    fn get_schema_by_id(&self, schema_id: SchemaId, collection_id: CollectionId) -> SchemaJson {
        let collection = self.collections.get(&collection_id).expect("collection not exist");
        let schema = collection.schemas.get(&schema_id).expect("Schema does not exist");

        SchemaJson{
            name: schema.name,
            creator_id: schema.creator_id,
            collection_id: schema.collection_id,
            schema_id: schema.schema_id,
        }
    }

    fn get_schema_by_collection_id(
        &self,
        collection_id: CollectionId,
        offset: Option<U128>,
        limit: Option<u64>
    )->Vec<SchemaJson> {
        let collection = self.collections.get(&collection_id).expect("collection not exist");
        let start_index: u128 = offset.map(From::from).unwrap_or_default();
        assert!(
            (collection.schemas.len() as u128) > start_index,
            "Out of bounds, please use a smaller offset."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        collection.schemas
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(_schema_id, schema)| SchemaJson{
                name: schema.name,
                creator_id: schema.creator_id,
                collection_id: schema.collection_id,
                schema_id: schema.schema_id,
            })
            .collect()
    }
}