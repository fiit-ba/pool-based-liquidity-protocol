use ink_storage::traits::PackedLayout;
use ink_storage::traits::SpreadLayout;
use brush::traits::AccountId;
use brush::traits::Balance;
use brush::traits::Timestamp;
use brush::contracts::traits::ownable::*;
use brush::contracts::traits::psp34::*;
use brush::contracts::traits::psp34::extensions::metadata::*;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct LoanInfo{
    pub borrower: AccountId,
    pub collateral_token: AccountId,
    pub collateral_amount: Balance,
    pub borrow_token: AccountId,
    pub borrow_amount: Balance,
    pub liquidation_price: Balance,
    pub timestamp: Timestamp,
    pub already_liquidated: bool,
}

#[brush::wrapper]
pub type LoanRef = dyn Loan + Ownable + PSP34 + PSP34Metadata;

#[brush::trait_definition]
pub trait Loan: Ownable + PSP34 + PSP34Metadata{
    #[ink(message)]
    fn create_loan(&mut self, loan_info: LoanInfo) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn delete_loan(&mut self, owner: AccountId, loan_id: Id) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn update_loan(&mut self, loan_id: Id, borrow_amount: Balance, collateral_amount: Balance, timestamp: Timestamp) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP34Error>;
}
