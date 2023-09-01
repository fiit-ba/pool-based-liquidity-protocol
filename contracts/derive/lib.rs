// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate proc_macro;

#[allow(unused_imports)]
use brush_derive::declare_derive_storage_trait;

// PSP22
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(derive_psp22_storage, PSP22Storage, PSP22StorageField);
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(
    derive_psp22metadata_storage,
    PSP22MetadataStorage,
    PSP22MetadataStorageField
);
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(
    derive_psp22wrapper_storage,
    PSP22WrapperStorage,
    PSP22WrapperStorageField
);
#[cfg(feature = "psp22")]
declare_derive_storage_trait!(
    derive_psp22token_timelock_storage,
    PSP22TokenTimelockStorage,
    PSP22TokenTimelockStorageField
);

// PSP34
#[cfg(feature = "psp34")]
declare_derive_storage_trait!(derive_psp34_storage, PSP34Storage, PSP34StorageField);
#[cfg(feature = "psp34")]
declare_derive_storage_trait!(
    derive_psp34_metadata_storage,
    PSP34MetadataStorage,
    PSP34MetadataStorageField
);
#[cfg(feature = "psp34")]
declare_derive_storage_trait!(
    derive_psp34_enumerable_storage,
    PSP34EnumerableStorage,
    PSP34EnumerableStorageField
);

// PSP1155
#[cfg(feature = "psp1155")]
declare_derive_storage_trait!(derive_psp1155_storage, PSP1155Storage, PSP1155StorageField);
#[cfg(feature = "psp1155")]
declare_derive_storage_trait!(
    derive_psp1155_metadata_storage,
    PSP1155MetadataStorage,
    PSP1155MetadataStorageField
);

// AccessControl
#[cfg(feature = "access_control")]
declare_derive_storage_trait!(
    derive_access_control_storage,
    AccessControlStorage,
    AccessControlStorageField
);

// Ownable
#[cfg(feature = "ownable")]
declare_derive_storage_trait!(derive_ownable_storage, OwnableStorage, OwnableStorageField);

// PaymentSplitter
#[cfg(feature = "payment_splitter")]
declare_derive_storage_trait!(
    derive_payment_storage,
    PaymentSplitterStorage,
    PaymentSplitterStorageField
);

// ReentrancyGuard
#[cfg(feature = "reentrancy_guard")]
declare_derive_storage_trait!(
    derive_reentrancy_storage,
    ReentrancyGuardStorage,
    ReentrancyGuardStorageField
);

// Pausable
#[cfg(feature = "pausable")]
declare_derive_storage_trait!(pausable_storage, PausableStorage, PausableStorageField);

// TimelockController
#[cfg(feature = "timelock_controller")]
declare_derive_storage_trait!(
    derive_timelock_controller_storage,
    TimelockControllerStorage,
    TimelockControllerStorageField
);

// Proxy
#[cfg(feature = "proxy")]
declare_derive_storage_trait!(derive_proxy_storage, ProxyStorage, ProxyStorageField);
// Diamond
#[cfg(feature = "diamond")]
declare_derive_storage_trait!(derive_diamond_storage, DiamondStorage, DiamondStorageField);
#[cfg(feature = "diamond")]
declare_derive_storage_trait!(
    derive_diamond_loupe_storage,
    DiamondLoupeStorage,
    DiamondLoupeStorageField
);
