/*
 * Created on Fri May 07 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

//! This module includes modpack data structs.

use serde::{Deserialize, Serialize};

use super::{PackLink, PackSpec};

/// Pack version data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackVersionData {
    /// Version id
    pub id: u32,

    /// Version name
    pub name: String,

    /// Version download count
    pub installs: i64,

    /// Version play count
    pub plays: i64,

    /// Version type (Release)
    #[serde(rename = "type")]
    pub version_type: String,

    /// ???
    pub notification: String,

    /// Version spec
    #[serde(default)]
    #[serde(deserialize_with = "super::empty_str_spec_as_none")]
    pub specs: Option<PackSpec>,

    /// Last info update time(?)
    pub refreshed: u32,

    /// Last version update time(?)
    pub updated: u32,

    /// Additional links
    pub links: Vec<PackLink>,

    /// Parent modpack id
    pub parent: u32,

    /// Launch dependencies
    pub targets: Vec<PackTarget>,

    /// File list
    pub files: Vec<PackFile>,
}

/// Pack file variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackFile {
    /// File type
    #[serde(rename = "type")]
    pub file_type: PackFileType,

    /// File information
    #[serde(flatten)]
    pub info: PackFileInfo,
}

/// Pack file types.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PackFileType {
    /// Mod file
    #[serde(rename = "mod")]
    Mod,

    /// Resource file
    #[serde(rename = "resource")]
    Resource,

    /// Config file
    #[serde(rename = "config")]
    Config,

    /// Script file
    #[serde(rename = "script")]
    Script,

    /// Curseforge overrides zip
    #[serde(rename = "cf-extract")]
    Overrides
}

/// Pack file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackFileInfo {
    /// File id
    pub id: u32,

    /// File name with extension
    pub name: String,

    /// true if the file is optional
    pub optional: bool,

    /// Relative path for file
    pub path: String,

    /// true if only client side file
    pub clientonly: bool,

    /// true if only server side file
    pub serveronly: bool,

    /// File sha1 hash (hex)
    pub sha1: String,

    /// File size (byte)
    pub size: i64,

    // pub tags: Vec<PackTag>,
    /// File update time
    pub updated: u32,

    /// File url
    pub url: String,

    /// File version
    pub version: FileVersion,
}

/// File version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileVersion {

    Numberic(u32),
    Semantic(String)

}

/// Pack launch dependency item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackTarget {
    /// Target id(?)
    pub id: u32,

    /// Target name
    pub name: String,

    /// Target type (modloader, game, ...)
    #[serde(rename = "type")]
    pub target_type: String,

    /// Target update time
    pub updated: u32,

    /// Target version
    pub version: String,
}
