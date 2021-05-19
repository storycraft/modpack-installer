/*
 * Created on Fri May 07 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use serde::{Deserialize, Serialize};

/// API search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {

    /// Pack id list
    #[serde(default)]
    pub packs: Vec<u32>,

    /// Curseforge pack id list
    #[serde(default)]
    pub curseforge: Vec<u32>,

    /// Item limits
    #[serde(default)]
    pub limit: u32,

    /// Total items found
    #[serde(default)]
    pub total: u32,

    /// Search time
    #[serde(default)]
    pub refreshed: u32,

}
