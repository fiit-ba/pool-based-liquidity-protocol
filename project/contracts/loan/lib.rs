#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract is responsible for operations with loans.
#[brush::contract]
pub mod loan {
    /// imports of libraries and traits needed
    use ink_prelude::string::String;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;
    use brush::modifiers;
    use brush::contracts::ownable::*;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::metadata::*;
    use liquidity_pool_protocol::traits::loan::*;

    /// Defining storage and its components and also deriving needed imports to our storage.
    #[ink(storage)]
    #[derive(OwnableStorage, PSP34Storage, PSP34MetadataStorage, SpreadAllocate)]
    /// Our contracts variables are stored in this struct.
    pub struct LoanContract{
        #[OwnableStorageField]
        ownable: OwnableData,
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        /// We will store Id of last loan here, it will serve as an aid to calculating the next Id.
        last_loan_id: Id,
        /// We will store mapping pair LoanId <--> Loan here.
        existing_loan_list: Mapping<Id, LoanInfo>,
    }

    /// We inherit the implementation of the ownable trait.
    impl Ownable for LoanContract {}
    /// We inherit the implementation of the PSP34 trait.
    impl PSP34 for LoanContract {}
    // We inherit the implementation of the PSP34Metadata trait.
    impl PSP34Metadata for LoanContract {}
    /// We implement functions declared in Loan trait.
    impl Loan for LoanContract {
        /// # Returns
        /// Returns a Ok(()) with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `mut loan_info` - mutable struct of type LoanInfo where info about loan is stored
        ///
        /// # Description
        /// create_loan function is declared in trait Loan and its function is to create new loan
        /// according to data received from arguments and storing info about it.
        #[ink(message)]
        /// Only_owner modifier is used to access this function just to authorized user.
        fn create_loan(&mut self, loan_info: LoanInfo) -> Result<(), PSP34Error> {
            // We get next free Id for our loan.
            let loan_id = self.get_new_loan_id()?;
            // We check if the loan with Id already exists, it should not but we have to handle it.
            if self.existing_loan_list.get(&loan_id).is_some(){
                return Err(PSP34Error::Custom(String::from("Loan with this Id already exists")))
            }
            // We insert pair loan_id <--> loan_info to the mapping.
            self.existing_loan_list.insert(&loan_id, &loan_info);
            // We mint the PSP34 token to the Account borrowing the asset
            // Return value of this function is return value which we get from _mint_to function.
            self._mint_to(loan_info.borrower, loan_id)
        }

        /// # Returns
        /// Returns a Ok(()) with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `caller` - AccountId of user calling this operation
        /// * `loan_id` Id of loan to be deleted
        ///
        /// # Description
        /// delete_loan function is declared in trait Loan
        /// its function is to delete outdated loan burn PSP34 token from the owner
        /// Only_owner modifier is used to access this function just to authorized user.
        #[ink(message)]
        fn delete_loan(&mut self, caller: AccountId, loan_id: Id) -> Result<(), PSP34Error> {
            // We delete loan from the mapping.
            self.existing_loan_list.remove(&loan_id);
            // We burn this PSP34 loan token from user.
            // Return value of this function is return value which we get from _burn_from function.
            self._burn_from(caller, loan_id)
        }

        /// # Returns
        /// Returns a Ok(()) with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` Id of loan to be deleted
        /// * `borrow_amount` updated Balance of borrowed asset
        /// * `collateral_amount` updated Balance of collateral asset
        /// * `timestamp` Timestamp of this action
        ///
        /// # Description
        /// update_loan function is declared in trait Loan
        /// its function is to update loan info after repay action
        /// Only_owner modifier is used to access this function just to authorized user.
        #[ink(message)]
        fn update_loan(&mut self, loan_id: Id, borrow_amount: Balance, collateral_amount: Balance, timestamp: Timestamp) -> Result<(), PSP34Error> {
            // We call helpers function which updates info needed to be updated.
            // Return value of this function is return value which we get from update_loan_info function.
            self.update_loan_internal(loan_id, borrow_amount, collateral_amount, timestamp)
        }

        /// # Returns
        /// Returns a Ok(()) with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` Id of loan to be liquidated
        ///
        /// # Description
        /// liquidate_loan function is declared in trait Loan
        /// its function is to liquidate loan if possible
        /// Only_owner modifier is used to access this function just to authorized user.
        #[modifiers(only_owner)]
        #[ink(message)]
        fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error> {
            // We call helpers function which do liquidation proccess for us.
            // Return value of this function is return value which we get from liquidate_loan function.
            self.liquidate_loan_internal(loan_id)
        }

        /// # Returns
        /// Returns a LoanInfo with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` Id of loan to get info about
        ///
        /// # Description
        /// get_loan_info function is declared in trait Loan
        /// its function is to get loan info by Id
        #[ink(message)]
        fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP34Error> {
            // Try to find loan in mapping by its key.
            let loan_info = self.existing_loan_list.get(&loan_id);
            // Check if we got something.
            if loan_info.is_none() {
                return Err(PSP34Error::Custom(String::from("Loan does not exist")))
            }
            // Return unwrapped loan info.
            Ok(loan_info.unwrap())
        }
    }

    /// This contract will contain constructor and helper functions for trait defined functions.
    impl LoanContract {
        /// # Returns
        /// Returns Self
        ///
        /// # Description
        /// Constructor for initializing our loan contract.
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            // Use initialize_contract from ink_lang::codegen library.
            ink_lang::codegen::initialize_contract(|instance: &mut LoanContract| {
                // Set last loan id to 1.
                instance.last_loan_id = Id::U8(1u8);
                instance._set_attribute(Id::U8(1u8), String::from("LoanPSP34").into_bytes(), String::from("LPSP34").into_bytes());
            })
        }

        /// # Returns
        /// Returns a Ok(()) with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` Id of loan to be updated
        /// * `borrow_amount` updated Balance of borrowed asset
        /// * `collateral_amount` updated Balance of collateral asset
        /// * `timestamp` Timestamp of this action
        ///
        /// # Description
        /// Helper function to update loan info.
        fn update_loan_internal(&mut self, loan_id: Id, borrow_amount: Balance, collateral_amount: Balance, timestamp: Timestamp) -> Result<(), PSP34Error> {
            // Get loan from mapping by its Id.
            let loan_info = self.existing_loan_list.get(&loan_id);
            // Check if the loan got exists.
            if loan_info.is_none(){
                return Err(PSP34Error::Custom(String::from("Loan with this Id does not exist!")))
            }
            // Make mutable version of the loan and update its Balance and timestamp variables.
            let mut loan_info_editable = loan_info.unwrap();
            loan_info_editable.borrow_amount = borrow_amount;
            loan_info_editable.collateral_amount = collateral_amount;
            loan_info_editable.timestamp = timestamp;
            // Add new loan to mappping.
            self.existing_loan_list.insert(&loan_id, &loan_info_editable);
            // Return Ok
            Ok(())
        }

        //// # Returns
        /// Returns a Ok(()) with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` Id of loan to be liquidated
        ///
        /// # Description
        /// Helper function to liquidate a loan.
        fn liquidate_loan_internal(&mut self, loan_id: Id) -> Result<(), PSP34Error> {
            // Get loan from mapping by its Id.
            let loan_info = self.existing_loan_list.get(&loan_id);
            // Check if the loan got exists.
            if loan_info.is_none() {
                return Err(PSP34Error::Custom(String::from("Loan with this Id does not exist")))
            }
            // Make mutable version of the loan and update its already_liquidated variable.
            let mut loan_info_editable = loan_info.unwrap();
            loan_info_editable.already_liquidated = true;
            // Add new loan to mappping.
            self.existing_loan_list.insert(&loan_id, &loan_info_editable);
            // Return Ok
            Ok(())
        }

        //// # Returns
        /// Returns a available Id with success and PSP34Error otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        ///
        /// # Description
        /// Helper function to get usable Id.
        fn get_new_loan_id(&mut self) -> Result<Id, PSP34Error> {
            match self.last_loan_id {
                // In the case value received from match is Id:U8 type
                Id::U8(v) => {
                    // We look for upper limit, we are checking for overflows.
                    if v == u8::MAX {
                        return Err(PSP34Error::Custom(String::from("Maximum loan ID reached!")))
                    }
                    // New maximum = previous value + 1
                    self.last_loan_id = Id::U8(v + 1);
                }
                // In default case, return Err.
                _ => {
                    return Err(PSP34Error::Custom(String::from("Invalid input format!")))
                }
            };
            // Clone incremented value to local variable and return it.
            let last = self.last_loan_id.clone();
            Ok(last)
        }
    }
}
