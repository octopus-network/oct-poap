use crate::types::Uuid;
use crate::ActivityId;
use crate::*;
use near_sdk::{AccountId, Timestamp};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VActivity {
    Current(Activity),
}

impl VActivity {
    /// Upgrades from other versions to the currently used version.
    #[allow(unused)]
    pub fn into_current(self, activity_id: &ActivityId) -> Activity {
        match self {
            VActivity::Current(activity) => activity,
        }
    }
}

impl From<Activity> for VActivity {
    fn from(activity: Activity) -> Self {
        VActivity::Current(activity)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Activity {
    uuid: Uuid,
    creator_id: AccountId,
    title: String,
    description: String,
    time: Timestamp,
    media_url: String,
}
