#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract is representing shares of individual pools.
/// This contract will not be used directly but its instances will be created when new supported assets are added.
#[brush::contract]
pub mod btoken {
    /// imports of libraries and traits needed
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use brush::modifiers;
    use brush::contracts::ownable::*;
    use brush::contracts::psp22::extensions::burnable::*;
    use brush::contracts::psp22::extensions::metadata::*;
    use brush::contracts::psp22::extensions::mintable::*;
    use liquidity_pool_protocol::traits::btoken::*;

    /// Defining storage and its components and also deriving needed imports to our storage.
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, OwnableStorage, PSP22MetadataStorage, PSP22Storage)]
    /// Our contracts variables are stored in this struct.
    pub struct BTokenContract{
        #[OwnableStorageField]
        ownable: OwnableData,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    /// We inherit the implementation of the ownable trait.
    impl Ownable for BTokenContract {}
    /// We inherit the implementation of the PSP22 trait.
    impl PSP22 for BTokenContract {}
    /// We implement ownable version for the implementation of the PSP22Burnable trait.
    impl PSP22Burnable for BTokenContract {
        /// # Returns
        /// Returns a Ok(()) with success and PSP22Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `account` AccountId of where the tokens are burnt from
        /// * `amount` how many tokens do we burn
        ///
        /// # Description
        /// Burn specific number of tokens from account specified.
        #[ink(message)]
        /// Only_owner modifier is used to access this function just to authorized user.
        #[modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            // We burn the PSP22 tokens from the Account
            // Return value of this function is return value which we get from _burn_from function.
            self._burn_from(account, amount)
        }
    }
    /// We inherit the implementation of the PSP22Metadata trait.
    impl PSP22Metadata for BTokenContract {}
    /// We implement ownable version for the implementation of the PSP22Mintable trait.
    impl PSP22Mintable for BTokenContract {
        /// # Returns
        /// Returns a Ok(()) with success and PSP22Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `account` AccountId of where the tokens are minted to
        /// * `amount` how many tokens do we mint
        ///
        /// # Description
        /// Mint specific number of tokens to account specified.
        #[ink(message)]
        /// Only_owner modifier is used to access this function just to authorized user.
        #[modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            // We mint the PSP22 tokens to the Account
            // Return value of this function is return value which we get from _mint function.
            self._mint(account, amount)
        }
    }
    /// We inherit the implementation of our PoolShares trait
    ///  Compiler has to check then if we implemented all trait functions declared.
    impl BToken for BTokenContract {}
    /// Implementation of PoolShares smart contract.
    impl BTokenContract {
        /// # Returns
        /// Returns Self
        ///
        /// # Arguments
        /// * `name` optional String for specifying our shares name in Metadata
        /// * `symbol` optional String for specifying our shares symbol in Metadata
        ///
        /// # Description
        /// Constructor for initializing our contract.
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut BTokenContract| {
                // Set metadata variables.
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = 18;
                instance._init_with_owner(instance.env().caller());
            })
        }
    }
}
