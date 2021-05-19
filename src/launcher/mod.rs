/*
 * Created on Mon May 10 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

pub mod profile;

use std::path::PathBuf;

use directories::BaseDirs;

pub const LAUNCHER_PROFILE_FILE: &'static str = "launcher_profiles.json";

/// Platform specific default minecraft dir
pub fn default_minecraft_dir() -> PathBuf {
    let base_dir = BaseDirs::new();

    match base_dir {
        Some(base_dir) => {
            #[cfg(target_os = "windows")]
            return base_dir.data_dir().join(".minecraft");

            #[cfg(target_os = "macos")]
            return base_dir.data_dir().join("minecraft");

            #[cfg(target_os = "linux")]
            return base_dir.home_dir().join(".minecraft");

            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            return "".into();
        },

        None => "".into()
    }
}
