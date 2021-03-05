#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ink_plutocratic_hosting {
    #[ink(storage)]
    pub struct ContentTracker {
        value: bool,
    }

    impl ContentTracker {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let ink_plutocratic_hosting = ContentTracker::default();
            assert_eq!(ink_plutocratic_hosting.get(), false);
        }

        #[test]
        fn it_works() {
            let mut ink_plutocratic_hosting = ContentTracker::new(false);
            assert_eq!(ink_plutocratic_hosting.get(), false);
            ink_plutocratic_hosting.flip();
            assert_eq!(ink_plutocratic_hosting.get(), true);
        }
    }
}
