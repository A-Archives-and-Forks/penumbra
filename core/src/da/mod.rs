/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    SPDX-FileCopyrightText: 2025-2026 Shomy
*/
pub mod protocol;
mod scatter;
pub mod xflash;
pub mod xml;
pub use protocol::{DAProtocol, DAProtocolParams, DownloadProtocol};
pub use scatter::{ScatterFile, ScatterOp, ScatterPartition};
pub use xflash::XFlash;
pub use xml::Xml;
