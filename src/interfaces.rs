use crate::{ActivityCreatorId, ActivityId};
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata};
use near_sdk::AccountId;

pub trait OwnerAction {
    fn set_contract_metadata(&mut self, metadata: NFTContractMetadata) -> bool;

    fn add_creator_whitelist(&mut self, account_ids: Vec<AccountId>);

    fn remove_creator_whitelist(&mut self, account_ids: Vec<AccountId>);
}

pub trait ActivityCreatorAction {
    fn create_activity_token_metadata(&mut self, token_metadata: TokenMetadata) -> ActivityId;

    fn update_activity_token_metadata(
        &mut self,
        activity_id: ActivityId,
        token_metadata: TokenMetadata,
    ) -> TokenMetadata;

    fn nft_mint(&mut self, activity_id: u32, nft_owner_ids: Vec<AccountId>);
}

pub trait View {
    fn get_creator_whitelist(&self) -> Vec<AccountId>;

    fn get_activity_token_metadata(&self, activity_id: ActivityId) -> Option<TokenMetadata>;

    fn get_creator_activities(&self, creator_id: AccountId) -> Vec<(ActivityId, TokenMetadata)>;

    fn get_activity(&self, activity_id: ActivityId) -> Option<(ActivityCreatorId, TokenMetadata)>;

    fn get_activities(
        &self,
        from_index: u32,
        limit: u32,
    ) -> Vec<(ActivityCreatorId, TokenMetadata)>;
}
