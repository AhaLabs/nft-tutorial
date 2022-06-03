use near_contract_standards::non_fungible_token::{
    metadata::{NFTContractMetadata, TokenMetadata},
    NonFungibleToken, Token, TokenId,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LazyOption,
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue,
};

mod owner;
mod standards;
mod util;
mod views;

use standards::*;
use util::log_mint;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub(crate) tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let metadata = NFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "Simple".to_string(),
            symbol: "SIM".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    #[payable]
    pub fn nft_mint(&mut self, token_owner_id: AccountId) -> Token {
        self.nft_mint_ungaurded(&token_owner_id, self.signer_is_owner())
    }

    fn nft_mint_ungaurded(&mut self, token_owner_id: &AccountId, mint_for_free: bool) -> Token {
        let refund_account = if mint_for_free {
            None
        } else {
            Some(token_owner_id.clone())
        };
        let token_id = (self.nft_total_supply().0 + 1).to_string();

        let token_metadata = TokenMetadata {
            title: Some(format!("Token number {}", token_id)),
            description: Some("Sample nft token".to_string()),
            // TODO media
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: Some(env::block_timestamp_ms().to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        let token = self.tokens.internal_mint_with_refund(
            token_id,
            token_owner_id.clone(),
            Some(token_metadata),
            refund_account,
        );

        // Emit mint event log
        log_mint(token_owner_id, &vec![token.clone()]);
        token
    }

    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "Method is private to owner")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::signer_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.tokens.owner_id.as_str()
    }
}
