use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

pub type ActivityCreatorId = AccountId;
pub type Uuid = u64;
pub type ActivityId = U64;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActivityView {
    pub activity_id: ActivityId,
    pub creator_id: ActivityCreatorId,
    pub token_metadata: TokenMetadata,
    pub ids: Vec<AccountId>,
    pub nft_ids: Vec<TokenId>,
}
