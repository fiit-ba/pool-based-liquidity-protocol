#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract is responsible for the main logic of the protocol
/// It uses both loan and btoken contracts and directly interact with them.
#[brush::contract]
pub mod liquidity_pool_manager{
    /// imports of libraries and traits needed
    use brush::contracts::access_control::*;
    use brush::contracts::pausable::*;
    use brush::contracts::traits::psp22::*;
    use brush::contracts::traits::psp34::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;
    use ink_lang::codegen::Env;
    use brush::traits::AccountIdExt;
    use brush::traits::ZERO_ADDRESS;
    use ink_lang::ToAccountId;
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    use brush::modifiers;
    use liquidity_pool_protocol::traits::btoken::BTokenRef;
    use liquidity_pool_protocol::traits::loan::LoanRef;
    use liquidity_pool_protocol::traits::loan::LoanInfo;
    use liquidity_pool_protocol::traits::liquidity_pool_manager::*;
    use loan_contract::loan::LoanContractRef;
    use btoken_contract::btoken::BTokenContractRef;

    /// Constants trait_definition
    /// YEAR in seconds
    pub const YEAR: Timestamp = 31536000;
    /// Admin role
    pub const ADMIN: RoleType = 0;
    /// default APY
    pub const APY: Balance = 1000;

    /// Defining storage and its components and also deriving needed imports to our storage.
    #[ink(storage)]
    #[derive(Default, AccessControlStorage, PausableStorage, SpreadAllocate)]
    pub struct PoolManagerContract {
        #[AccessControlStorageField]
        access: AccessControlData,
        #[PausableStorageField]
        pause: PausableData,

        // code hash of btoken contract representing pool shares
        pub btoken_contract_code_hash: Hash,
        /// the AccountId of the loan
        pub loan_account: AccountId,
        /// Mapping of conversion rates bewtween two assets.
        /// 1 unit of currency1 = X of currency2
        pub conversion_rates: Mapping<(AccountId, AccountId), Balance>,
        /// Mapping from asset address to the address where the assets were lended.
        pub assets_lended: Mapping<AccountId, AccountId>,
        /// Mapping from asset address to its btoken address.
        pub asset_btoken: Mapping<AccountId, AccountId>,
        /// Mapping from btoken asset address to its asset address.
        pub btoken_asset: Mapping<AccountId, AccountId>,
        /// mapping of collateral_accepted, AccountId of collateral -> bool: accepted/not accepted
        pub collateral_accepted: Mapping<AccountId, bool>,
    }

    /// We inherit the implementation of the access control trait.
    impl AccessControl for PoolManagerContract {}
    /// We inherit the implementation of the pausable trait.
    impl Pausable for PoolManagerContract {}

    impl PoolManager for PoolManagerContract {
        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of the asset that is lended
        /// * `amount` - the Balance of how much asset is lended
        ///
        /// # Description
        /// lend is an important function which handles the process of lending into the contract
        #[ink(message, payable)]
        #[modifiers(when_not_paused)]
        fn lend(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), PoolManagerError>{
            // get the address of the caller = address of the lender
            let lender = self.env().caller();
            // get the address of the contract
            let contract = self.env().account_id();
            // get the allowance for the contract
            let allowance = PSP22Ref::allowance(&asset_address, lender, contract);
            // check if the allowance is sufficient
            if allowance < amount{
                return Err(PoolManagerError::InsufficientAllowance)
            }
            // get the balance of the user
            let user_balance = PSP22Ref::balance_of(&asset_address, lender);
            // check if the balance is sufficient
            if user_balance < amount {
                return Err(PoolManagerError::InsufficientBalance)
            }
            // get the amount of the asset already in the contract
            let total_asset = self.get_total_asset(asset_address)?;
            // get the amount of the btoken of the asset
            let total_btoken = self.get_total_btoken(asset_address)?;
            // transfer the asset from the user to the contract
            PSP22Ref::transfer_from_builder(&asset_address, lender, contract, amount, Vec::<u8>::new())
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            // check if some asset is already in the contract
            let btoken_amount;
            if total_asset == 0 {
                btoken_amount = amount;
            }
            // else recalculate btoken distribution
            else {
                btoken_amount = (amount * total_btoken) / total_asset;
            }
            let btoken_address = self.get_btoken_from_asset(asset_address)?;

            // mint the btoken to the user
            BTokenRef::mint_builder(&btoken_address, lender, btoken_amount)
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            Ok(())
        }

        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `btoken_address` - AccountId of the btoken that user got for lending asset
        /// * `btoken_amount` - the Balance of how much btoken does he want to withdraw
        ///
        /// # Description
        /// withdraw is an important function which handles the process of withdrawing asset from the contract
        #[ink(message)]
        fn withdraw(&mut self, btoken_address: AccountId, btoken_amount: Balance) -> Result<(), PoolManagerError> {
            // save the address of the caller = address of the withdrawer
            let withdrawer = self.env().caller();
            // get the address of the contract
            let contract = self.env().account_id();
            // get asset address from btoken address
            let withdraw_asset = self.get_asset_from_btoken(btoken_address)?;
            // get total_asset = assets in the contract + assets lended
            let total_asset = self.get_total_asset(withdraw_asset)?;
            // get total_supply of btokens
            let total_supply = PSP22Ref::total_supply(&btoken_address);
            // calculate how much does user able to withdraw
            let withdraw_amount = btoken_amount * total_asset / total_supply;
            // check how much does contract have in the reserve
            let withdrawable_amount = PSP22Ref::balance_of(&withdraw_asset, contract);
            // if user want to withdraw more than contract has, it is not possible
            if withdraw_amount > withdrawable_amount{
                return Err(PoolManagerError::InsufficientPoolBalance)
            }
            // burn btokens
            BTokenRef::burn_builder(&btoken_address, withdrawer, btoken_amount)
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            // give assets to user back
            PSP22Ref::transfer_builder(&withdraw_asset, withdrawer, withdraw_amount, Vec::<u8>::new())
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            Ok(())
        }


        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of the asset that is borrowed
        /// * `collateral_address` - AccountId of the asset that is colateralized
        /// * `amount` - the Balance of how much asset is borrowed
        ///
        /// # Description
        /// borrow is an important function which handles the process of borrowing assets from the contract
        #[ink(message)]
        #[modifiers(when_not_paused)]
        fn borrow(&mut self, asset_address: AccountId, collateral_address: AccountId, amount: Balance) -> Result<(), PoolManagerError> {
            // get the address of the caller = address of the borrower
            let borrower = self.env().caller();
            // get the address of the contract
            let contract = self.env().account_id();
            // get the allowance for the contract
            let allowance = PSP22Ref::allowance(&collateral_address, borrower, contract);
            // check if the allowance is sufficient
            if allowance < amount{
                return Err(PoolManagerError::InsufficientAllowance)
            }
            let user_balance = PSP22Ref::balance_of(&collateral_address, borrower);
            // check if the balance is sufficient
            if user_balance < amount {
                return Err(PoolManagerError::InsufficientBalance)
            }
            // check if the collateral is accepted
            if !self.get_collateral_acceptance(collateral_address) {
                return Err(PoolManagerError::CollateralNotSupported)
            }
            // get the btoken asset address
            let btoken_address = self.get_btoken_from_asset(asset_address)?;
            // calculate the deposited collateral price
            let price = self.get_conversion_rate(collateral_address, asset_address, amount);
            // set the liquidation price to 75%
            let liquidation_price = price * 75 / 100;
            // set the borrowed amount to 70%
            let borrow_amount = price * 70 / 100;
            // to be sure, check if borrow_ammount is not greater equal to liquidation price
            if borrow_amount >= liquidation_price {
                return Err(PoolManagerError::AmountNotSupported)
            }
            let asset_balance = PSP22Ref::balance_of(&asset_address, contract);
            if asset_balance < borrow_amount{
                return Err(PoolManagerError::InsufficientPoolBalance)
            }
            // transfer the collateral to the users address
            PSP22Ref::transfer_from_builder(&collateral_address, borrower, contract, amount, Vec::<u8>::new())
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            // create loan info
            let loan_info = LoanInfo{
                borrower: borrower,
                collateral_token: collateral_address,
                collateral_amount: amount,
                borrow_token: asset_address,
                borrow_amount: borrow_amount,
                liquidation_price: liquidation_price,
                timestamp: self.env().block_timestamp(),
                already_liquidated: false
            };
            // create loan
            LoanRef::create_loan(&(self.loan_account), loan_info)?;
            // transfer the asset to the borrower
            PSP22Ref::transfer_builder(&asset_address, borrower, borrow_amount, Vec::<u8>::new())
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            // mint borrow amount of the reserve token
            BTokenRef::mint_builder(&btoken_address, contract, borrow_amount)
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            Ok(())
        }

        /// # Returns
        /// Returns a bool with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` - Id of the loan to be repayed
        /// * `repay_amount` - Balance of the asset repayed to the contract
        ///
        /// # Description
        /// repay is an important function which handles the process of repaying borrowed assets to the contract
        /// true = repayed, false = already liquidated
        #[ink(message)]
        fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, PoolManagerError> {
            // get the address of the caller = address of the borrower
            let repayer = self.env().caller();
            // get the address of the contract
            let contract = self.env().account_id();
            // get the loan info
            let loan_info = LoanRef::get_loan_info(&(self.loan_account), loan_id.clone())?;
            // check if loan was not already liquidated, delete the loan in that case
            if loan_info.already_liquidated{
                LoanRef::delete_loan(&(self.loan_account), repayer, loan_id.clone())?;
                return Ok(false)
            }
            // get the allowance from the user
            let allowance = PSP22Ref::allowance(&loan_info.borrow_token, repayer, contract);
            // if user provided insufficient allowance, return error
            if allowance < repay_amount{
                return Err(PoolManagerError::InsufficientAllowance)
            }
            // get the balance of user
            let user_balance = PSP22Ref::balance_of(&loan_info.borrow_token, repayer);
            // if user has insufficient balance, return error
            if  user_balance < repay_amount {
                return Err(PoolManagerError::InsufficientBalance)
            }
            // calculate time elapsed since loan creation
            let timer = self.env().block_timestamp() - loan_info.timestamp;
            // calculate interest of the loan
            let interest = (APY * timer as Balance) / YEAR as Balance;
            // calculate how much user needs to repay
            let to_repay = (((loan_info.borrow_amount) * (10000 + interest)) / 10000) + 1;
            // get btoken asset addrees
            let btoken_address = self.get_btoken_from_asset(loan_info.borrow_token)?;
            // case if the user want to repay the whole loan
            if repay_amount >= to_repay {
                // transfer loaned asset back to the contract
                PSP22Ref::transfer_from_builder(&loan_info.borrow_token, repayer, contract, to_repay, Vec::<u8>::new())
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
                // transfer collateralized asset back to the user
                PSP22Ref::transfer_builder(&loan_info.collateral_token, repayer, loan_info.collateral_amount, Vec::<u8>::new())
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
                // delete repayed loan
                LoanRef::delete_loan(&(self.loan_account), repayer, loan_id)?;
                // burn btokens
                BTokenRef::burn_builder(&btoken_address, self.env().caller(), loan_info.borrow_amount)
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
            }
            // case if the user want to repay loan partially
            else {
                // transfer loaned asset back to the contract
                PSP22Ref::transfer_from_builder(&loan_info.borrow_token, repayer, contract, repay_amount, Vec::<u8>::new())
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
                // calculate proprtion of
                let to_return = (repay_amount * loan_info.collateral_amount) / to_repay;
                // transfer collateralized asset back to the user
                PSP22Ref::transfer_builder(&loan_info.collateral_token, repayer, to_return, Vec::<u8>::new())
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
                // update loan info
                LoanRef::update_loan(
                    &(self.loan_account),
                    loan_id.clone(),
                    to_repay - repay_amount,
                    self.env().block_timestamp().into(),
                    (loan_info.collateral_amount - to_return).try_into().unwrap(),
                )?;
                // mint btoken to the contract
                BTokenRef::mint_builder(&btoken_address, contract, to_repay - repay_amount - loan_info.borrow_amount)
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
            }
            Ok(true)
        }

        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `loan_id` - Id of the loan to be liquidated
        ///
        /// # Description
        /// liquidate is an important function which handles the process of liquidating unhealthy assets
        #[ink(message)]
        fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PoolManagerError> {
            // get the address of the caller = address of the liquidator
            let liquidator = self.env().caller();
            // get the loan account
            let loan_account = self.loan_account;
            // get loan info of the loan id
            let loan_info = LoanRef::get_loan_info(&loan_account, loan_id.clone())?;
            // if the loan is already liquidated, return error
            if loan_info.already_liquidated {
                return Err(PoolManagerError::LoanAlreadyLiquidated)
            }
            // get collateral price
            let price = self.get_conversion_rate(loan_info.collateral_token, loan_info.borrow_token, loan_info.collateral_amount);
            // check if the loan is unhealthy
            if price <= loan_info.liquidation_price {
                // calculate reward for liquidating
                let reward = (loan_info.collateral_amount * 1000) / 100000;
                // transfer collateral to liquidator
                PSP22Ref::transfer_builder(&loan_info.collateral_token, liquidator, reward,Vec::<u8>::new())
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap()?;
                // use liquidation loan function from Loan
                LoanRef::liquidate_loan(&loan_account, loan_id.clone())?;
            }
            else {
                return Err(PoolManagerError::LoanUnliquidable)
            }
        Ok(())
        }

        /// # Returns
        /// Returns a bool
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset we want to know if it is lendable to the contract.
        ///
        /// # Description
        /// get_asset_acceptance is helper function
        /// It is responsible for getting bool representation of acceptance of the specific asset.
        #[ink(message)]
        fn get_asset_acceptance(&mut self, asset_address: AccountId) -> bool {
            let acceptance = !self.asset_btoken.get(&asset_address).unwrap_or(ZERO_ADDRESS.into()).is_zero();
            acceptance
        }

        /// # Returns
        /// Returns a bool
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset we want to know if it is collateralizable
        ///
        /// # Description
        /// get_collateral_acceptance is helper function
        /// It is responsible for getting bool representation of acceptance of the specific collateral asset.
        #[ink(message)]
        fn get_collateral_acceptance(&mut self, asset_address: AccountId) -> bool {
            self.collateral_accepted.get(&asset_address).unwrap_or(false)
        }

        /// # Returns
        /// Returns a Balance with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset from which we want to get its total Balance
        ///
        /// # Description
        /// get_total_asset is helper function
        /// It is responsible for getting total asset Balance for given asset addresss (lended + in pool).
        #[ink(message)]
        fn get_total_asset(&mut self, asset_address: AccountId) -> Result<Balance, PoolManagerError> {
            // get asset lended from asset address
            let asset_lended = self.assets_lended.get(&asset_address).unwrap_or(ZERO_ADDRESS.into());
            // check if we got something
            if asset_lended.is_zero() {
                return Err(PoolManagerError::AssetNotFound)
            }
            // get AccountId of executed contract
            let contract = self.env().account_id();
            // calculate assets in contract
            let available = PSP22Ref::balance_of(&asset_address, contract);
            // calculate assets lended
            let unavailable = PSP22Ref::balance_of(&asset_lended, contract);
            Ok(available + unavailable)
        }

        /// # Returns
        /// Returns a Balance with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset from which we want to get btoken bounded total Balance
        ///
        /// # Description
        /// get_total_btoken is helper function
        /// It is responsible for getting total Balance for given pool addresss.
        #[ink(message)]
        fn get_total_btoken(&mut self, asset_address: AccountId) -> Result<Balance, PoolManagerError>{
            let btoken_address = self.get_btoken_from_asset(asset_address)?;
            let total = PSP22Ref::total_supply(&btoken_address);
            Ok(total)
        }

        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset to be allowed
        /// * `name` optional String for specifying our shares name in Metadata
        /// * `symbol` optional String for specifying our shares symbol in Metadata
        /// * `decimal` u8 for specifying our shares decimals in Metadata
        ///
        /// # Description
        /// set_asset_allowance is ADMIN function to make asset lendable and borrowable
        #[ink(message, payable)]
        #[modifiers(only_role(ADMIN))]
        fn set_asset_allowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError> {
            // check for asset allowance
            if self.get_asset_acceptance(asset_address) {
                return Err(PoolManagerError::AssetAlreadySupported)
            }
            // instantiate the btoken for lended assets and pool token for reserves
            let btoken_address = self.instantiate_btoken_contract("BTokenShares", "BTKS");
            let reserves_address = self.instantiate_btoken_contract("TokenReserves", "TRS");
            // accept the asset by inserting it into the mappings
            self.assets_lended.insert(&asset_address, &reserves_address);
            self.asset_btoken.insert(&asset_address, &btoken_address);
            self.btoken_asset.insert(&btoken_address, &asset_address);
            Ok(())
        }

        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset to be allowed as a collateral
        ///
        /// # Description
        /// set_collateral_allowance is ADMIN function to make asset collateralizable for the loans
        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        fn set_collateral_allowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError> {
            // check for collateral allowance
            if self.get_collateral_acceptance(asset_address) {
                return Err(PoolManagerError::CollateralAlreadySupported)
            }
            // accept the collateral by inserting it into the mapping
            self.collateral_accepted.insert(&asset_address, &true);
            Ok(())
        }

        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset to be disallowed
        ///
        /// # Description
        /// set_asset_disallowance is ADMIN function to make asset unlendable
        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        fn set_asset_disallowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError> {
            // obtain btoken address
            let btoken_address = self.get_btoken_from_asset(asset_address)?;
            // get btoken status and check for pool emptiness
            let asset_balance = PSP22Ref::balance_of(&asset_address, self.env().account_id());
            let btoken_balance = PSP22Ref::balance_of(&btoken_address, self.env().account_id());
            if asset_balance > 0 || btoken_balance > 0{
                return Err(PoolManagerError::PoolIsNotEmpty)
            }
            self.assets_lended.remove(&asset_address);
            self.asset_btoken.remove(&asset_address);
            self.btoken_asset.remove(&btoken_address);
            Ok(())
        }

        /// # Returns
        /// Returns a Ok(()) with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset to be disallowed as a collateral
        ///
        /// # Description
        /// set_collateral_disallowance is ADMIN function to make asset uncollateralizable
        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        fn set_collateral_disallowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError> {
            // check for collateral disallowance
            if !self.get_collateral_acceptance(asset_address) {
                return Err(PoolManagerError::CollateralAlreadyUnsupported)
            }
            // disallow the collateral by setting its value to false
            self.collateral_accepted.insert(&asset_address, &false);
            Ok(())
        }

        /// # Returns
        /// Without a return value
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_from` - AccountId of source asset from which the conversion takes place
        /// * `asset_to` - AccountId of destionation asset to which the conversion takes place
        /// * `exchange_rate` - Balance of 1 asset_from to X asset_to conversion
        ///
        /// # Description
        /// set_conversion_rate function is helper function responsible for setting conversion rates between assets
        /// It says us how many of asset_to assets do we get for 1 asset_from asset.
        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        fn set_conversion_rate(&mut self, asset_from: AccountId, asset_to: AccountId, exchange_rate: Balance) -> Result<(), PoolManagerError> {
            self.conversion_rates.insert((&asset_from, &asset_to), &exchange_rate);
            Ok(())
        }

        /// # Returns
        /// Returns a Balance
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_from` - AccountId of source asset from which the conversion takes place
        /// * `asset_to` - AccountId of destionation asset to which the conversion takes place
        /// * `amount_from` - Balance of how much of asset_from we want to transfer to asset_to
        ///
        /// # Description
        /// get_conversion_rate is helper function responsible for getting conversion rates between assets
        /// It says us how many of asset_to assets do we get for amount_from of asset_from asset.
        #[ink(message)]
        fn get_conversion_rate(&mut self, asset_from: AccountId, asset_to: AccountId, amount_from: Balance) -> Balance {
            // get conversion rate from mapping
            let price = self.conversion_rates.get((&asset_from, &asset_to)).unwrap_or(0) * amount_from;
            price
        }

        /// # Returns
        /// Returns an AccountId with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `btoken_address` - AccountId of bToken from which we want to get its bounded asset address
        ///
        /// # Description
        /// get_asset_from_btoken is helper function
        /// It is responsible for getting asset address which is bound to btoken address.
        #[ink(message)]
        fn get_asset_from_btoken(&mut self, btoken_address: AccountId) -> Result<AccountId, PoolManagerError>{
            // get asset address from pool_token_address
            let asset_address = self.btoken_asset.get(&btoken_address).unwrap_or(ZERO_ADDRESS.into());
            // check if we got something
            if asset_address.is_zero() {
                return Err(PoolManagerError::AssetNotFound)
            }
            Ok(asset_address)
        }

        /// # Returns
        /// Returns an AccountId with success and PoolManagerError otherwise
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `asset_address` - AccountId of asset from which we want to get its btoken address
        ///
        /// # Description
        /// get_btoken_from_asset is helper function
        /// It is responsible for getting address of btoken for supplying asset addres into the contract
        #[ink(message)]
        fn get_btoken_from_asset(&mut self, asset_address: AccountId) -> Result<AccountId, PoolManagerError> {
            // get btoken address from asset address
            let btoken_address = self.asset_btoken.get(&asset_address).unwrap_or(ZERO_ADDRESS.into());
            // check if we got something
            if btoken_address.is_zero() {
                return Err(PoolManagerError::AssetNotFound)
            }
            Ok(btoken_address)
        }
    }

    /// This contract will contain constructor and helper functions for trait defined functions.
    impl PoolManagerContract {
        /// # Returns
        /// Returns Self
        ///
        /// # Arguments
        /// * `loan_hash` - hash of the loan deployed smart contract
        /// * `btoken_hash` - hash of the btoken deployed smart contract
        ///
        /// # Description
        /// Constructor for initializing our contract.
        #[ink(constructor, payable)]
        pub fn new(loan_hash: Hash, btoken_hash: Hash) -> Self{
            // Use initialize_contract from ink_lang::codegen library.
            ink_lang::codegen::initialize_contract(|instance: &mut PoolManagerContract| {
                // to variable caller set AccountId that initialized the contract
                let caller = instance.env().caller();
                // function from AccessControl trait, caller is added to admin list
                instance._init_with_admin(caller);
                // get hash of btoken contract to local struct
                instance.btoken_contract_code_hash = btoken_hash;
                // instantiate loan contract
                let loan = LoanContractRef::new()
                    .endowment(0)
                    .code_hash(loan_hash)
                    .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                    .instantiate()
                    .unwrap();
                instance.loan_account = loan.to_account_id();
            })
        }

        /// # Returns
        /// Returns an AccountId
        ///
        /// # Arguments
        /// * `&mut self` - used so we can mutate variables of self component
        /// * `contract_name` optional String for specifying name in Metadata
        /// * `contract_symbol` optional String for specifying symbol in Metadata
        ///
        /// # Description
        /// instantiate_btoken_contract is helper function that creates instance of btoken with given data
        fn instantiate_btoken_contract(&mut self, contract_name: &str, contract_symbol: &str) -> AccountId {
            let code_hash = self.btoken_contract_code_hash;
            let (hash, _) = ink_env::random::<ink_env::DefaultEnvironment>(contract_name.as_bytes()).expect("Failed to get salt");
            let hash = hash.as_ref();
            let contract = BTokenContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
                    .endowment(0)
                    .code_hash(code_hash)
                    .salt_bytes(&hash[..4])
                    .instantiate()
                    .unwrap();
            contract.to_account_id()
        }
    }
}
