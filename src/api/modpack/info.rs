/*
 * Created on Fri May 07 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

//! This module includes modpack information structs.

use serde::{Deserialize, Serialize};

use super::{PackLink, PackSpec, PackTag};

/// A ModPack struct contains modpack informations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModPack {
    /// Modpack id
    pub id: u32,

    /// The name of the modpack
    pub name: String,

    /// Short description about this modpack
    pub synopsis: Option<String>,

    /// Full description of the modpack
    pub description: Option<String>,

    /// true if this modpack is featured modpack
    pub featured: bool,

    /// Modpack download count
    pub installs: i64,

    /// Modpack play count
    pub plays: i64,

    /// Last info update time(?)
    pub refreshed: u32,

    /// Last pack update time(?)
    pub updated: u32,

    /// Pack type (release)
    #[serde(rename = "type")]
    pub pack_type: String,

    /// ???
    pub notification: String,

    /// Modpack tags
    pub tags: Vec<PackTag>,

    /// Modpack rating
    pub rating: PackRating,

    /// Pack versions
    pub versions: Vec<PackVersion>,

    /// Pack arts
    #[serde(rename = "art")]
    pub arts: Vec<PackArt>,

    /// Pack authors
    pub authors: Vec<PackAuthor>,

    /// Pack links
    pub links: Vec<PackLink>,
}

impl ModPack {

    /// Convert author list to single string
    pub fn author_str(&self) -> String {
        self.authors
        .iter()
        .map(|item| item.name.clone())
        .collect::<Vec<String>>()
        .join(", ")
    }

}

/// Modpack art
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackArt {
    /// Art type
    #[serde(rename = "type")]
    pub art_type: PackArtType,

    /// Art information
    #[serde(flatten)]
    pub info: PackArtInfo,
}

/// Modpack art types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PackArtType {
    /// Logo image
    #[serde(rename = "logo")]
    Logo,

    /// Splash type is usually widescreen images.
    #[serde(rename = "splash")]
    Splash,

    /// Square type is usually used on pack icons, etc.
    #[serde(rename = "square")]
    Square,
}

/// Modpack art info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackArtInfo {
    /// Art id
    pub id: u32,

    /// True if art is compressed(?)
    pub compressed: bool,

    /// Image width
    pub width: u32,

    /// Image height
    pub height: u32,

    /// Image sha1 hex
    pub sha1: String,

    /// Image size
    pub size: i64,

    /// Image update time
    pub updated: u32,

    /// Image url
    pub url: String,
}

/// Modpack rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackRating {
    /// Rating id(?)
    pub id: i32,

    pub age: u16,
    pub alcoholdrugs: bool,
    pub configured: bool,
    pub frightening: bool,
    pub gambling: bool,
    pub language: bool,
    pub nuditysexual: bool,
    pub sterotypeshate: bool,
    pub verified: bool,
    pub violence: bool,
}

/// Modpack versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackVersion {
    /// Pack version id
    pub id: u32,

    /// Pack version.
    /// This field should follow semantic versioning.
    pub name: String,

    /// Last update time(?)
    pub updated: u32,

    /// Version spec requirement
    #[serde(default)]
    #[serde(deserialize_with = "super::empty_str_spec_as_none")]
    pub specs: Option<PackSpec>,

    /// Version type (release)
    #[serde(rename = "type")]
    pub version_type: String,
}

/// Modpack author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackAuthor {
    /// Author id(?)
    pub id: u32,

    /// Author name
    pub name: String,

    /// Author type (team, ...)
    #[serde(rename = "type")]
    pub author_type: String,

    /// Author website
    pub website: String,

    /// Last update time(?)
    pub updated: u32,
}
