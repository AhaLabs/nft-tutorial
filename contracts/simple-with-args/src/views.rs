use crate::*;

#[near_bindgen]
impl Contract {
    /// Current contract owner
    pub fn owner(&self) -> AccountId {
        self.tokens.owner_id.clone()
    }
}
