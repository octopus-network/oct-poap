use crate::interfaces::View;
use crate::*;

impl View for Contract {
    fn get_creator_whitelist(&self) -> Vec<AccountId> {
        self.creator_whitelist.iter().collect()
    }

    fn get_activity_token_metadata(&self, activity_id: ActivityId) -> Option<TokenMetadata> {
        self.activity_token_metadata.get(&activity_id)
    }

    fn get_creator_activities(&self, creator_id: AccountId) -> Vec<(ActivityId, TokenMetadata)> {
        self.activities_by_creators
            .get(&creator_id)
            .map(|ids| {
                ids.iter()
                    .map(|id| (id, self.activity_token_metadata.get(&id).unwrap()))
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn get_activity(&self, activity_id: ActivityId) -> Option<(ActivityCreatorId, TokenMetadata)> {
        self.activities
            .get(&activity_id)
            .and_then(|id| Some((id, self.activity_token_metadata.get(&activity_id).unwrap())))
    }

    fn get_activities(
        &self,
        from_index: u32,
        limit: u32,
    ) -> Vec<(ActivityCreatorId, TokenMetadata)> {
        self.activities
            .iter()
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|(activity_id, activity_creator_id)| {
                (
                    activity_creator_id,
                    self.activity_token_metadata.get(&activity_id).unwrap(),
                )
            })
            .collect()
    }
}
