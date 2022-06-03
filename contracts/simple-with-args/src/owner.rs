use crate::*;

#[near_bindgen]
impl Contract {
    // Owner private methods

    /// @allow ["::owner"]
    pub fn transfer_ownership(&mut self, new_owner: AccountId) -> bool {
        self.assert_owner();
        env::log_str(&format!(
            "{} transfers ownership to {}",
            self.tokens.owner_id, new_owner
        ));
        self.tokens.owner_id = new_owner;
        true
    }
}
