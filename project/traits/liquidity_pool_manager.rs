use brush::traits::AccountId;
use brush::traits::Balance;
use brush::contracts::traits::access_control::*;
use brush::contracts::traits::pausable::*;
use brush::contracts::traits::psp22::*;
use brush::contracts::traits::psp34::*;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PoolManagerError {
    PSP22Error(PSP22Error),
    PSP34Error(PSP34Error),
    AccessControlError(AccessControlError),
    PausableError(PausableError),
    AssetNotFound,
    BTokenNotFound,
    AssetAlreadySupported,
    AssetNotSupported,
    PoolIsNotEmpty,
    CollateralAlreadySupported,
    CollateralAlreadyUnsupported,
    CollateralNotSupported,
    InsufficientAllowance,
    InsufficientBalance,
    InsufficientPoolBalance,
    AmountNotSupported,
    NotTheLoanOwner,
    LoanAlreadyLiquidated,
    LoanUnliquidable
}

#[brush::wrapper]
pub type PoolManagerRef = dyn PoolManager + AccessControl + Pausable;

#[brush::trait_definition]
pub trait PoolManager: AccessControl + Pausable{
    #[ink(message, payable)]
    fn lend(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn withdraw(&mut self, btoken_address: AccountId, btoken_amount: Balance) -> Result<(), PoolManagerError> ;

    #[ink(message)]
    fn borrow(&mut self, asset_address: AccountId, collateral_address: AccountId, amount: Balance) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, PoolManagerError>;

    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn get_asset_acceptance(&mut self, asset_address: AccountId) -> bool;

    #[ink(message)]
    fn get_collateral_acceptance(&mut self, asset_address: AccountId) -> bool;

    #[ink(message)]
    fn get_total_asset(&mut self, asset_address: AccountId) -> Result<Balance, PoolManagerError>;

    #[ink(message)]
    fn get_total_btoken(&mut self, asset_address: AccountId) -> Result<Balance, PoolManagerError>;

    #[ink(message)]
    fn get_conversion_rate(&mut self, asset_from: AccountId, asset_to: AccountId, amount_from: Balance) -> Balance;

    #[ink(message, payable)]
    fn set_asset_allowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn set_collateral_allowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn set_asset_disallowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn set_collateral_disallowance(&mut self, asset_address: AccountId) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn set_conversion_rate(&mut self, asset_from: AccountId, asset_to: AccountId, exchange_rate: Balance) -> Result<(), PoolManagerError>;

    #[ink(message)]
    fn get_asset_from_btoken(&mut self, btoken_address: AccountId) -> Result<AccountId, PoolManagerError>;

    #[ink(message)]
    fn get_btoken_from_asset(&mut self, asset_address: AccountId) -> Result<AccountId, PoolManagerError>;
}

impl From<AccessControlError> for PoolManagerError {
    fn from(access: AccessControlError) -> Self {
        PoolManagerError::AccessControlError(access)
    }
}

impl From<PausableError> for PoolManagerError {
    fn from(access: PausableError) -> Self {
        PoolManagerError::PausableError(access)
    }
}

impl From<PSP22Error> for PoolManagerError {
    fn from(error: PSP22Error) -> Self {
        PoolManagerError::PSP22Error(error)
    }
}

impl From<PSP34Error> for PoolManagerError {
    fn from(error: PSP34Error) -> Self {
        PoolManagerError::PSP34Error(error)
    }
}
