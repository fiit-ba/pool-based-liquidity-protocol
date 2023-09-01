use brush::contracts::traits::ownable::*;
use brush::contracts::traits::psp22::*;
use brush::contracts::traits::psp22::extensions::burnable::*;
use brush::contracts::traits::psp22::extensions::metadata::*;
use brush::contracts::traits::psp22::extensions::mintable::*;

#[brush::wrapper]
pub type BTokenRef = dyn Ownable + PSP22 + PSP22Burnable + PSP22Metadata + PSP22Mintable;

#[brush::trait_definition]
pub trait BToken: Ownable + PSP22 + PSP22Burnable + PSP22Metadata + PSP22Mintable {}
