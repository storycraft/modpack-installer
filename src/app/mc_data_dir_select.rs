/*
 * Created on Tue May 11 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::{
    fs,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use dialoguer::Input;

use crate::launcher::{LAUNCHER_PROFILE_FILE, default_minecraft_dir, profile::LauncherProfile};

/// Minecraft data directory select screen
pub struct MCDataDirSelect {
    /// Default data directory
    default_dir: String,
}

impl MCDataDirSelect {
    pub fn new(default_dir: String) -> Self {
        Self { default_dir }
    }

    /// Create with default minecraft directory
    pub fn new_default() -> Self {
        Self {
            default_dir: default_minecraft_dir().to_string_lossy().to_string(),
        }
    }

    /// Select directory and returns (PathBuf, LauncherProfile) tuple.
    pub fn select(self, prompt: Option<&str>) -> Result<(PathBuf, LauncherProfile), io::Error> {
        let mut sel = Input::<String>::new();

        if let Some(prompt) = prompt {
            sel.with_prompt(prompt);
        }

        sel.with_initial_text(self.default_dir);

        loop {
            let path_str = sel.interact_text()?;
            let path = Path::new(&path_str);

            let launcher_profile_path = path.join(LAUNCHER_PROFILE_FILE);

            match fs::File::open(&launcher_profile_path) {
                Ok(launcher_profile) => {
                    match serde_json::from_reader(BufReader::new(launcher_profile)) {
                        Ok(profile) => return Ok((path.into(), profile)),

                        Err(err) => {
                            println!(
                                "{}",
                                console::style(format!(
                                    "{} Error: {}",
                                    "Cannot read launcher profile. Choose another directory.",
                                    err
                                ))
                                .red()
                            );
                        }
                    }
                }

                Err(err) => {
                    println!(
                        "{}",
                        console::style(format!(
                            "{} Error: {}",
                            "Invalid minecraft data directory", err
                        ))
                        .red()
                    );
                }
            }
        }
    }
}
