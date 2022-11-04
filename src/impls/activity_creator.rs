use crate::interfaces::ActivityCreatorAction;
use crate::Contract;
use crate::*;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::{env, AccountId};

#[near_bindgen]
impl ActivityCreatorAction for Contract {
    fn create_activity_token_metadata(&mut self, token_metadata: TokenMetadata) -> ActivityId {
        let activity_id = self.assign_activity_id();
        let activity_creator = env::predecessor_account_id();
        self.activities.insert(&activity_id, &activity_creator);
        self.activities_by_creators.insert(
            &activity_creator,
            &Vector::new(StorageKey::ActivitiesByCreator {
                creator_id: activity_creator.clone(),
            }),
        );
        self.activity_token_metadata
            .insert(&activity_id, &token_metadata);
        activity_id
    }

    fn update_activity_token_metadata(
        &mut self,
        activity_id: ActivityId,
        token_metadata: TokenMetadata,
    ) -> TokenMetadata {
        self.activities
            .get(&activity_id)
            .expect(format!("The activity(#{}) hasn't been created!", activity_id).as_str());

        // The activity should be created before update token metadata.
        // So the old token metadata should be exist.
        self.activity_token_metadata
            .insert(&activity_id, &token_metadata)
            .unwrap()
    }

    fn nft_mint(&mut self, activity_id: u32, nft_owner_ids: Vec<AccountId>) {
        for nft_owner_id in nft_owner_ids {
            let token_id = format!("{}:{}", activity_id, nft_owner_id).to_string();

            if self.token.owner_by_id.get(&token_id).is_some() {
                log!(
                    "{} already done mint nft of activity #{}, jump mint for {}.",
                    nft_owner_id,
                    activity_id,
                    nft_owner_id
                );
                continue;
            }

            self.token.internal_mint(token_id, nft_owner_id, None);
        }
    }
}
