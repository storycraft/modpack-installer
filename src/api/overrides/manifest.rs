/*
 * Created on Wed May 12 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use serde::{Deserialize, Serialize};

/// Overrides package manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackManifest {

    /// Manifest type
    pub manifest_type: String,

    /// Manifest version
    pub manifest_version: u32,

    /// Optional overrides directory name
    pub overrides: Option<String>,

    /// Semantic pack version
    pub version: String,

    /// Author name
    pub author: String,

    /// Pack description
    pub description: String,

    /// File list
    pub files: Vec<PackFile>,

    /// Minecraft configuration
    pub minecraft: PackMC

}

/// Modpack file of manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackFile {

    /// Pack id
    #[serde(rename = "projectID")]
    pub project_id: u32,

    /// Curseforge file id
    #[serde(rename = "fileID")]
    pub file_id: u32,

    /// If presents and false the file is optional
    pub required: Option<bool>,

}

/// Modpack minecraft configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMC {

    #[serde(rename = "modLoaders")]
    pub modloaders: Vec<PackModLoader>,

    /// mc version
    pub version: String,

}

/// Modpack modloader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackModLoader {

    /// Loader id
    pub id: String,

    /// If presents and true the loader is primary
    pub primary: Option<bool>

}
