use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContentRecord {
    pub price: Balance,
    pub content: String,
    pub owner: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContentTracker {
    values: LookupMap<String, ContentRecord>,
    contract_owner: AccountId,
}

impl Default for ContentTracker {
    fn default() -> Self {
        // TODO verify if env is initialized in default call
        let contract_owner = env::predecessor_account_id();
        Self {
            values: LookupMap::new(b"v".to_vec()),
            contract_owner,
        }
    }
}

#[near_bindgen]
impl ContentTracker {
    /// Gets content at a given route.
    pub fn get_route(&self, route: String) -> Option<String> {
        self.values.get(&route).map(|v| v.content)
    }

    /// Purchases a route based on funds sent to the contract.
    #[payable]
    pub fn purchase(&mut self, route: String, content: String) {
        let deposit = env::attached_deposit();
        assert!(deposit > 0);
        if let Some(entry) = self.values.get(&route) {
            assert!(
                deposit > entry.price,
                "Not enough deposit to purchase route, price: {} deposit: {}",
                entry.price,
                deposit
            );

            // Refund purchase to existing owner
            Promise::new(entry.owner).transfer(entry.price);
        }

        // Update record for the contract state.
        self.values.insert(
            &route,
            &ContentRecord {
                price: deposit,
                content,
                owner: env::predecessor_account_id(),
            },
        );
    }

    /// Allows owner of the contract withdraw funds.
    pub fn withdraw(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.contract_owner);

        // Send the contract funds to the contract owner
        Promise::new(self.contract_owner.clone()).transfer(env::account_balance());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    fn get_context(name: impl ToString, is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(name.to_string())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn basic_initialize() {
        let context = get_context("bob", false);
        testing_env!(context);
        let contract = ContentTracker::default();
        assert!(contract.get_route("test".to_string()).is_none());
    }

    // * this is inconsistently failing tests to a seg fault
    // #[test]
    // // #[should_panic]
    // fn no_deposit_fail() {
    //     let context = get_context("bob", false);
    //     testing_env!(context);
    //     let mut contract = ContentTracker::default();

    //     // Should fail because no deposit attached
    //     std::panic::catch_unwind(move || {
    //         contract.purchase("troute".to_string(), "tcontent".to_string());
    //     })
    //     .unwrap_err();
    // }

    #[test]
    fn purchase_and_replace() {
        let mut context = get_context("bob", false);
        println!("name: {}", context.signer_account_id);
        testing_env!(context.clone());
        let mut contract = ContentTracker::default();

        context.attached_deposit = 2;
        testing_env!(context.clone());
        contract.purchase("troute".to_string(), "tcontent".to_string());
        assert_eq!(
            contract.get_route("troute".to_string()),
            Some("tcontent".to_string())
        );

        // Try purchasing same route with same amount
        context.predecessor_account_id = "alice".to_string();
        context.attached_deposit = 3;
        testing_env!(context.clone());
        contract.purchase("troute".to_string(), "new content".to_string());
        assert_eq!(
            contract.get_route("troute".to_string()),
            Some("new content".to_string())
        );

        // Contract owner
        context.predecessor_account_id = "bob".to_string();
        testing_env!(context.clone());
        contract.withdraw();
    }
}
