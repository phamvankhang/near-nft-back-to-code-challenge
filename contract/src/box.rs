use crate::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UnBoxResult {
    pub name: String,
    pub tokens: Vec<Token>,
}

pub trait Box {
    //approve an account ID to transfer a token on your behalf
    fn un_box(
        &mut self,
        token_id: TokenId
    ) -> UnBoxResult;

}

#[near_bindgen]
impl Box for Contract {

    #[payable]
    fn un_box(&mut self, token_id: TokenId) -> UnBoxResult {

        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        let token = self.tokens.get(&token_id).expect("token not exist");

        assert_eq!(&token.owner_id, caller_id, "You is not token owner");

        assert_eq!(&token.nft_type, "BOX".to_string(), "This NFT cannot unbox");

        let unbox_rate = token.metadata.unbox_rate;

        // TODO: create template with unbox rate

        let results = Vec::new();

        refund_deposit(env::storage_usage() - initial_storage_usage);

        UnBoxResult {
            name: template.name,
            tokens: results,
        }
    }

}