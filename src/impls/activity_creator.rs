use crate::interfaces::ActivityCreatorAction;
use crate::Contract;
use crate::*;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::{env, AccountId};

impl Contract {
    pub(crate) fn assert_creator(&self, activity_id: &ActivityId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.activities.get(&activity_id).expect("No such activity"),
            "Must be activity creator."
        )
    }
}

#[near_bindgen]
impl ActivityCreatorAction for Contract {
    #[payable]
    fn create_activity(&mut self, token_metadata: TokenMetadata) -> ActivityId {
        let activity_creator = env::predecessor_account_id();

        assert!(
            self.creator_whitelist.contains(&activity_creator),
            "{} is not in whitelist.",
            activity_creator
        );

        let activity_id = self.assign_activity_id();
        let prev_storage = env::storage_usage();

        self.activities.insert(&activity_id, &activity_creator);

        let mut creator_activities = self
            .activities_by_creators
            .get(&activity_creator)
            .unwrap_or(vec![]);
        creator_activities.push(activity_id);
        self.activities_by_creators
            .insert(&activity_creator, &creator_activities);

        self.activity_token_metadata
            .insert(&activity_id, &token_metadata);

        self.internal_check_storage(prev_storage);

        activity_id
    }

    #[payable]
    fn update_activity(
        &mut self,
        activity_id: ActivityId,
        token_metadata: TokenMetadata,
    ) -> TokenMetadata {
        self.assert_creator(&activity_id);

        let prev_storage = env::storage_usage();

        self.activities
            .get(&activity_id)
            .expect(format!("The activity(#{}) hasn't been created!", activity_id.0).as_str());

        // The activity should be created before update token metadata.
        // So the old token metadata should be exist.
        let token_metadata = self
            .activity_token_metadata
            .insert(&activity_id, &token_metadata)
            .unwrap();

        self.internal_check_storage(prev_storage);

        token_metadata
    }

    #[payable]
    fn nft_mint(&mut self, activity_id: ActivityId, nft_owner_ids: Vec<AccountId>) {
        self.assert_creator(&activity_id);

        let mut set = self
            .activity_tokens
            .get(&activity_id)
            .unwrap_or(UnorderedSet::new(StorageKey::ActivityTokensInner {
                activity_id: activity_id.clone(),
            }));
        for nft_owner_id in nft_owner_ids {
            let token_id = format!("{}:{}", activity_id.0, nft_owner_id).to_string();

            if self.token.owner_by_id.get(&token_id).is_some() {
                log!(
                    "{} already done mint nft of activity #{}, jump mint for {}.",
                    nft_owner_id,
                    activity_id.0,
                    nft_owner_id
                );
                continue;
            }

            self.token
                .internal_mint(token_id.clone(), nft_owner_id, None);
            set.insert(&token_id);

            self.token_activity.insert(&token_id, &activity_id);
        }

        self.activity_tokens.insert(&activity_id, &set);
    }
}
