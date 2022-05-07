use crate::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TemplateJson {
    token_series_id: TokenSeriesId,
    metadata: TokenMetadata,
    creator_id: AccountId,
    royalty: HashMap<AccountId, u32>,
    transaction_fee: Option<U128>
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Template {
    pub name: AccountId, // collection name equal account
    pub creator_id: AccountId,
    pub created_at: TimestampSec,
    pub token_metadata: TokenMetadata,
    pub schema_id: usize,
    pub collection_id: CollectionId,
    pub max_supply: Option<u64>,
    pub is_mintable: bool,
    pub tokens: UnorderedSet<TokenId>,
    pub attributes: Option<UnorderedMap<TemplateId, String>>,
    pub transferable: bool,
    pub burnable: bool,
    pub issued: usize,
}

pub trait TemplateTrait {
    //approve an account ID to transfer a token on your behalf
    fn create_template(
        &mut self,
        creator_id: AccountId,
        name: String,
        schema_id: SchemaId,
        collection_id: CollectionId,
        token_metadata: TokenMetadata
    )->Template;

}

#[near_bindgen]
impl TemplateTrait for Contract {
    #[payable]
    fn create_template(&mut self,
                       creator_id: AccountId,
                       name: String,
                       schema_id: SchemaId,
                       collection_id: CollectionId,
                       token_metadata: TokenMetadata
    ) ->Template {

        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();
        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        if creator_id.is_some() {
            assert_eq!(creator_id.unwrap().to_string(), caller_id, "Paras: Caller is not creator_id");
        }

        let template_id = format!("{}", (self.tokens_by_template_id.len() + 1));

        assert!(
            self.tokens_by_template_id.get(&template_id).is_none(),
            "Paras: duplicate template_id"
        );

        let title = token_metadata.title.clone();
        assert!(title.is_some(), "Paras: token_metadata.title is required");


        let mut total_perpetual = 0;
        let mut total_accounts = 0;
        let royalty_res: HashMap<AccountId, u32> = if let Some(royalty) = royalty {
            for (k , v) in royalty.iter() {
                if !is_valid_account_id(k.as_bytes()) {
                    env::panic("Not valid account_id for royalty".as_bytes());
                };
                total_perpetual += *v;
                total_accounts += 1;
            }
            royalty
        } else {
            HashMap::new()
        };

        assert!(total_accounts <= 10, "Paras: royalty exceeds 10 accounts");

        assert!(
            total_perpetual <= 9000,
            "Paras Exceeds maximum royalty -> 9000",
        );

        self.tokens_by_template_id.insert(&template_id, &Template{
            name,
            token_metadata: token_metadata.clone(),
            schema_id,
            collection_id,
            creator_id: caller_id.to_string(),
            tokens: UnorderedSet::new(
                StorageKey::TokensByTemplateInner {
                    token_series: template_id.clone(),
                }
                    .try_to_vec()
                    .unwrap(),
            ),
            is_mintable: true,
            created_at: to_sec(env::block_timestamp()),
            max_supply: 0
        });

        // set market data transaction fee
        let current_transaction_fee = self.calculate_current_transaction_fee();
        self.market_data_transaction_fee.transaction_fee.insert(&template_id, &current_transaction_fee);

        env::log(
            json!({
                "type": "nft_create_series",
                "params": {
                    "template_id": template_id,
                    "token_metadata": token_metadata,
                    "creator_id": caller_id,
                    "royalty": royalty_res,
                    "transaction_fee": &current_transaction_fee.to_string()
                }
            })
                .to_string()
                .as_bytes(),
        );

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        TemplateJson{
            token_series_id: (),
            template_id,
            metadata: token_metadata,
            creator_id: caller_id.into(),
            royalty: royalty_res,
            transaction_fee: Some(current_transaction_fee.into())
        }
    }

}