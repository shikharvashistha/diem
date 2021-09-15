// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{access_path::AccessPath, transaction::Version};
use anyhow::Result;

// TODO combine with ConfigStorage
pub trait MoveStorage {
    /// Returns a vector of Move resources as serialized byte array
    /// Order of resource returned matches the order of `access_path`
    fn fetch_resource(&self, access_path: AccessPath) -> Result<Vec<u8>>;

    /// Returns a vector of Move resources as serialized byte array from a
    /// specified version of the database
    /// Order of resources returned matches the order of `access_path`
    fn fetch_resource_at_version(
        &self,
        access_path: AccessPath,
        version: Version,
    ) -> Result<Vec<u8>>;

    /// Get the version on the latest transaction info.
    fn fetch_synced_version(&self) -> Result<Version>;
}
