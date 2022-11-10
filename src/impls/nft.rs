use crate::*;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[allow(unused_variables)]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        panic!("Can't transfer this nft!");
    }

    #[allow(unused_variables)]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        panic!("Can't transfer this nft!");
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        // self.token.nft_token(token_id)
        self.token.nft_token(token_id).and_then(|mut token| {
            self.internal_get_nft_metadata(&token.token_id)
                .and_then(|token_metadata| {
                    token.metadata = Some(token_metadata);
                    Some(token)
                })
        })
    }
}

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> U128 {
        self.token.nft_total_supply()
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        self.token
            .nft_tokens(from_index, limit)
            .into_iter()
            .map(|mut token| {
                token.metadata = self.internal_get_nft_metadata(&token.token_id);
                token
            })
            .collect()
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.token.nft_supply_for_owner(account_id)
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        self.token
            .nft_tokens_for_owner(account_id, from_index, limit)
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
