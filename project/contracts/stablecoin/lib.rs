#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract will represent the stablecoin which will be avalaible for lending and borrowing.
/// It will simulate stablecoin like DAI for project testability on the local node.
#[brush::contract]
pub mod stablecoin {
    /// imports of libraries and traits needed
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use brush::contracts::psp22::*;
    use brush::contracts::psp22::extensions::metadata::*;
    use liquidity_pool_protocol::traits::stablecoin::*;

    /// Defining storage and its components and also deriving needed imports to our storage.
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    /// Our contracts variables are stored in this struct.
    pub struct StableCoinContract {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    /// We inherit the implementation of the PSP22 trait.
    impl PSP22 for StableCoinContract {}
    /// We inherit the implementation of the PSP22Metadata trait.
    impl PSP22Metadata for StableCoinContract {}
    /// We inherit the implementation of our BToken trait
    ///  Compiler has to check then if we implemented all trait functions declared.
    impl StableCoin for StableCoinContract {}
    /// Implementation of BToken smart contract.
    impl StableCoinContract {
        /// # Returns
        /// Returns Self
        ///
        /// # Arguments
        /// * `name` optional String for specifying our coin name in Metadata
        /// * `symbol` optional String for specifying our coin symbol in Metadata
        ///
        /// # Description
        /// Constructor for initializing our contract.
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut StableCoinContract| {
                // Set metadata variables.
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = 18;
                let total_supply = 1_000_000 * 10_u128.pow(18);
                // Mint initial supply to the caller.
                assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            })
        }
    }
}
