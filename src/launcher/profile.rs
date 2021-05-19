/*
 * Created on Mon May 10 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

//! Minecraft launcher profiles declarations

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Vanilla launcher json structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProfile {
    /// Launcher game profiles.
    /// Key is random md5 hash or profile name (old).
    pub profiles: HashMap<String, GameLaunchProfile>,

    /// Launcher profile version information
    pub launcher_version: LauncherVersion,

    /// Put everything else we don't need here
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Launcher version struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherVersion {
    /// Launcher profile format
    pub format: u32,

    /// Semantic version string
    pub name: String,

    /// Game profile format
    pub profiles_format: u32,
}

/// Game launch profile struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameLaunchProfile {
    /// Date string with ISO format
    pub created: Option<String>,

    /// Last use date with ISO format
    pub last_used: Option<String>,

    /// Custom game directory
    pub game_dir: Option<String>,

    /// Custom java arguments
    pub java_args: Option<String>,

    /// Version to use launch the game
    pub last_version_id: String,

    /// Profile icon
    ///
    /// default: Furnace
    ///
    /// You can also use base64 format 128x128 png image as icon.
    pub icon: Option<String>,

    /// Profile name
    pub name: String,

    /// Profile type.
    ///
    /// Custom profile should use type "custom"
    #[serde(rename = "type")]
    pub profile_type: String,

    /// Put everything else we don't need here
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
