/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    SPDX-FileCopyrightText: 2025-2026 Shomy
*/
#![feature(trait_alias)]

pub mod activity;
mod auth;
pub mod da;
pub mod device;
pub mod error;
#[cfg(not(feature = "no_exploits"))]
pub mod exploit;
pub mod macros;
pub mod utilities;

pub use core::auth::{AuthManager, SignData, SignPurpose, SignRequest, Signer};
pub use core::log_buffer::{DeviceLog, OnPush};
pub use core::seccfg::LockFlag;
pub use core::storage::{
    EmmcPartition,
pub mod port;
mod preloader;
pub mod storage;
    Gpt,
    Partition,
    PartitionKind,
    RpmbRegion,
    Storage,
    StorageKind,
    StorageType,
    UfsPartition,
};

pub use da::protocol::{BootMode, DAProtocol, DownloadProtocol};
pub use da::{DA, DAEntryRegion, DAFile, DAType, XFlash, Xml};
pub use device::{Device, DeviceBuilder};

const VERSION: &str = env!("CARGO_PKG_VERSION");
