#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ink_plutocratic_hosting {
    #[allow(unused_imports)]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{PackedLayout, SpreadLayout},
    };

    /// Emitted whenever a route is purchased
    #[ink(event)]
    pub struct RoutePurchased {
        #[ink(topic)]
        route: String,
        #[ink(topic)]
        from: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct ContentRecord {
        pub price: Balance,
        pub content: String,
        pub owner: AccountId,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct ContentTracker {
        values: StorageHashMap<String, ContentRecord>,
        contract_owner: Lazy<AccountId>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Insufficient funds to purchase route.
        InsufficientDeposit,
        /// Failed to transfer funds to account.
        TransferFailed,
        /// Only contract owner can withdraw from the contract.
        InvalidOwner,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl ContentTracker {
        /// Creates a new content tracker contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Gets content at a given route.
        #[ink(message)]
        pub fn get_route(&self, route: String) -> Option<String> {
            self.values.get(&route).map(|v| v.content.clone())
        }

        /// Purchase route to store content.
        #[ink(message, payable)]
        pub fn purchase(&mut self, route: String, content: String) -> Result<()> {
            let deposit = self.env().transferred_balance();
            if deposit <= 0 {
                return Err(Error::InsufficientDeposit);
            }
            if let Some(entry) = self.values.get(&route) {
                if deposit <= entry.price {
                    return Err(Error::InsufficientDeposit);
                }

                self.env()
                    .transfer(entry.owner, entry.price)
                    .map_err(|_| Error::TransferFailed)?;
            }

            // Update record for the contract state.
            self.values.insert(
                route,
                ContentRecord {
                    price: deposit,
                    content,
                    owner: self.env().caller(),
                },
            );
            Ok(())
        }

        /// Allows owner of the contract withdraw funds.
        #[ink(message)]
        pub fn withdraw(&mut self) -> Result<()> {
            if self.env().caller() != *self.contract_owner {
                return Err(Error::InvalidOwner);
            }
            self.env()
                .transfer(*self.contract_owner, self.env().balance())
                .map_err(|_| Error::TransferFailed)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        const DEFAULT_CALLEE_HASH: [u8; 32] = [0x07; 32];
        const DEFAULT_ENDOWMENT: Balance = 1_000_000;
        const DEFAULT_GAS_LIMIT: Balance = 1_000_000;

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("off-chain environment should have been initialized already")
        }

        fn set_next_caller(caller: AccountId) {
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                caller,
                AccountId::from(DEFAULT_CALLEE_HASH),
                DEFAULT_ENDOWMENT,
                DEFAULT_GAS_LIMIT,
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])),
            )
        }

        #[ink::test]
        fn basic_initialize() {
            let default_accounts = default_accounts();

            set_next_caller(default_accounts.alice);
            let mut _contract = ContentTracker::new();
        }
    }
}
