use near_contract_standards::non_fungible_token::{events::NftMint, Token};
use near_sdk::AccountId;

pub fn log_mint(owner_id: &AccountId, tokens: &[Token]) {
    let token_ids = &tokens
        .iter()
        .map(|t| t.token_id.as_str())
        .collect::<Vec<&str>>();
    NftMint {
        owner_id,
        token_ids,
        memo: None,
    }
    .emit()
}
