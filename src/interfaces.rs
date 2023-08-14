use crate::types::ActivityView;
use crate::ActivityId;
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;

pub trait OwnerAction {
    fn set_owner(&mut self, owner: AccountId);

    fn set_contract_metadata(&mut self, metadata: NFTContractMetadata) -> bool;

    fn add_creator_whitelist(&mut self, account_ids: Vec<AccountId>);

    fn remove_creator_whitelist(&mut self, account_ids: Vec<AccountId>);
}

pub trait ActivityCreatorAction {
    fn create_activity(&mut self, token_metadata: TokenMetadata) -> ActivityId;

    fn update_activity(
        &mut self,
        activity_id: ActivityId,
        token_metadata: TokenMetadata,
    ) -> TokenMetadata;

    fn nft_mint(&mut self, activity_id: ActivityId, nft_owner_ids: Vec<AccountId>);
}

pub trait View {
    fn get_owner(&self) -> AccountId;

    fn get_creator_whitelist(&self) -> Vec<AccountId>;

    fn get_activity_token_metadata(&self, activity_id: ActivityId) -> Option<TokenMetadata>;

    fn get_creator_activities(&self, creator_id: AccountId) -> Vec<ActivityView>;

    fn get_activity(&self, activity_id: ActivityId) -> Option<ActivityView>;

    fn get_activities(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<ActivityView>;

    fn get_nft_metadata(&self, token_id: TokenId) -> Option<TokenMetadata>;

    fn contains_nft(&self, activity_id: ActivityId, nft_owner: AccountId) -> bool;
}
