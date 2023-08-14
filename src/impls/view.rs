use std::collections::HashSet;

use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;

use crate::interfaces::View;
use crate::types::ActivityView;
use crate::*;

#[near_bindgen]
impl View for Contract {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn get_creator_whitelist(&self) -> Vec<AccountId> {
        self.creator_whitelist.iter().collect()
    }

    fn get_activity_token_metadata(&self, activity_id: ActivityId) -> Option<TokenMetadata> {
        self.activity_token_metadata.get(&activity_id)
    }

    fn get_creator_activities(&self, creator_id: AccountId) -> Vec<ActivityView> {
        self.activities_by_creators
            .get(&creator_id)
            .map(|ids| {
                ids.into_iter()
                    .map(|id| self.get_activity(id).unwrap())
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn get_activity(&self, activity_id: ActivityId) -> Option<ActivityView> {
        self.activities.get(&activity_id).and_then(|creator_id| {
            let nft_ids = self
                .activity_tokens
                .get(&activity_id)
                .map(|ids| ids.to_vec())
                .unwrap_or(vec![]);
            let ids = nft_ids
                .iter()
                .map(|token_id| self.token.owner_by_id.get(&token_id).unwrap())
                .collect();

            Some(ActivityView {
                activity_id,
                creator_id,
                token_metadata: self.activity_token_metadata.get(&activity_id).unwrap(),
                ids,
                nft_ids,
            })
        })
    }

    fn get_activities(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<ActivityView> {
        self.activities
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(self.activities.len() as u32) as usize)
            .map(|(activity_id, _)| self.get_activity(activity_id).unwrap())
            .collect()
    }

    fn get_nft_metadata(&self, token_id: TokenId) -> Option<TokenMetadata> {
        self.internal_get_nft_metadata(&token_id)
    }

    fn contains_nft(&self, activity_id: ActivityId, nft_owner_id: AccountId) -> bool {
        let nft_token_id = format!("{}:{}", activity_id.0, nft_owner_id).to_string();
        self.token
            .tokens_per_owner
            .as_ref()
            .unwrap()
            .get(&nft_owner_id)
            .unwrap()
            .contains(&nft_token_id)
    }
}
