#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod dead_man_switch {
    /// Defines the storage of the contract.
    #[ink(storage)]
    struct DeadManSwitch {
        /// Every benefactor should send a heartbeat every `heartbeat_frequency` seconds.
        heartbeat_frequency: storage::Value<u64>,
        // TODO: Can't figure out how to store struct as hashmap value. Come back to it later.
        /*/// Store a mapping from benefactors AccountId to a Benefactor
        benefactors: storage::HashMap<AccountId, Benefactor>,*/
        /// Following 3 maps will have 1 entry each benefactor. This is ugly and should be replaced
        /// by one hashmap per benefactor where value is a struct.

        /// The amount of inheritance that it to be given to heir
        benefactor_balances: storage::HashMap<AccountId, Balance>,
        /// Heir's AccountId where inheritance will be transferred
        benefactor_heirs: storage::HashMap<AccountId, AccountId>,
        /// Last block number when the heartbeat was sent.
        /// XXX: Using Block number for now since can't find a way to access current time.
        benefactor_heartbeats: storage::HashMap<AccountId, BlockNumber>
    }

    // TODO: Can't figure out how to store struct as hashmap value. Come back to it later.
    /*struct Benefactor {
        /// The amount of inheritance that it to be given to heir
        my_balance: storage::Value<Balance>,
        /// Heir's AccountId where inheritance will be transferred
        heir_account: storage::Value<AccountId>,
        /// Last time when the heartbeat was sent
        last_hearbeat_at: storage::Value<Moment>
    }*/

    impl DeadManSwitch {
        /// Constructor that initializes the `heartbeat_frequency` value to the given `heartbeat_frequency`.
        #[ink(constructor)]
        fn new(&mut self, heartbeat_frequency: u64) {
            self.heartbeat_frequency.set(heartbeat_frequency);
        }

        /// Return the current heartbeat frequency. A benefactor should ping at least once in this
        /// duration to be considered alive.
        #[ink(message)]
        fn get_heartbeat_frequency(&self) -> u64 {
            *self.heartbeat_frequency
        }

        /// Register a new benefactor based on the caller's AccountId. Only register if the caller
        /// is not already registered and return true. Return false if already registered.
        #[ink(message)]
        fn register_benefactor(&mut self, heir_id: storage::Value<AccountId>, inheritance: storage::Value<Balance>) -> bool {
            let caller = self.env().caller();
            // Any of the 3 structs can be checked here
            match self.benefactor_heartbeats.get(&caller) {
                Some(_) => {
                    let caller_balance = self.env().transferred_balance();
                    if caller_balance < *inheritance {
                        // Caller doesn't have enough balance as he intends to leave in inheritance
                        self.env()
                            .emit_event(
                                BenefactorRegistrationFailed {
                                    benefactor: caller,
                                });
                        false
                    } else {
                        self.benefactor_balances.insert(caller, *inheritance);
                        self.benefactor_heirs.insert(caller, *heir_id);
                        self.update_heartbeat(caller);
                        self.env()
                            .emit_event(
                                NewBenefactor {
                                    benefactor: caller,
                                    heir: *heir_id,
                                    inheritance: *inheritance,
                                });
                        true
                    }
                }
                None => false
            }
        }

        /// A heartbeat sent by a benefactor
        /// Process the heartbeat if the benefactor is registered and return true. Return false otherwise.
        #[ink(message)]
        fn ping(&mut self) -> bool {
            let caller = self.env().caller();
            match self.benefactor_heartbeats.get(&caller) {
                Some(_) => {
                    self.update_heartbeat(caller);
                    true
                }
                None => false
            }
        }

        /// Update last received heartbeat of the caller to the current block number
        /// Fixme: Update the last heartbeat to current time. Not logging an event to avoid storage cost.
        fn update_heartbeat(&mut self, caller: AccountId) {
            let current_block_number = self.env().block_number();
            self.benefactor_heartbeats.insert(caller, current_block_number);
        }
    }

    #[ink(event)]
    struct NewBenefactor {
        #[ink(topic)]
        benefactor: AccountId,
        #[ink(topic)]
        heir: AccountId,
        inheritance: Balance,
    }

    #[ink(event)]
    struct BenefactorRegistrationFailed {
        #[ink(topic)]
        benefactor: AccountId,
    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[test]
        fn check_heartbeat_frequency_after_init() {
            // TODO: Check if the heartbeat frequency is set correctly
        }

        #[test]
        fn check_benefactor_registration_fails_for_already_registered() {
            // TODO:
        }

        #[test]
        fn check_benefactor_registration_fails_when_insufficient_balance() {
            // TODO: Check event as well
        }

        #[test]
        fn check_benefactor_registration_works_for_unregistered() {
            // TODO: Check event as well
        }

        #[test]
        fn check_ping_fails_for_unregistered() {
            // TODO:
        }

        #[test]
        fn check_ping_works_for_registered() {
            // TODO:
        }
    }
}
