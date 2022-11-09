use crate::interfaces::OwnerAction;
use crate::*;
use near_contract_standards::upgrade::Ownable;

#[near_bindgen]
impl Ownable for Contract {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}

#[near_bindgen]
impl OwnerAction for Contract {
    fn set_contract_metadata(&mut self, metadata: NFTContractMetadata) -> bool {
        self.assert_owner();
        self.metadata.set(&metadata)
    }

    fn add_creator_whitelist(&mut self, account_ids: Vec<AccountId>) {
        self.assert_owner();
        for account_id in account_ids {
            self.creator_whitelist.insert(&account_id);
        }
    }

    fn remove_creator_whitelist(&mut self, account_ids: Vec<AccountId>) {
        self.assert_owner();
        for account_id in account_ids {
            self.creator_whitelist.remove(&account_id);
        }
    }
}
