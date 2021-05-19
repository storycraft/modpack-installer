/*
 * Created on Sun May 09 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU Lesser General Public License v3.
 */

use std::io;

use dialoguer::Select;
use semver::Version;

use crate::api::modpack::info::PackVersion;

/// Modpack version select screen
pub struct PackVersionSelect {
    list: Vec<PackVersion>
}

impl PackVersionSelect {

    pub fn new(list: Vec<PackVersion>) -> Self {
        Self { list }
    }

    /// Display selection
    pub fn select(self, prompt: Option<&str>) -> Result<Option<PackVersion>, io::Error> {
        let mut list = self.list;

        if list.len() < 1 {
            return Ok(None);
        }

        list.sort_by(|ver1, ver2| {
            let ver1 = Version::parse(&ver1.name).unwrap_or(Version::new(1, 0, 0));
            let ver2 = Version::parse(&ver2.name).unwrap_or(Version::new(1, 0, 0));

            ver1.cmp(&ver2).reverse()
        });

        let index = {
            let mut sel = Select::new();
            sel.items(&list.iter().map(Self::version_desc).collect::<Vec<String>>()).paged(true).default(0);

            if let Some(prompt) = prompt {
                sel.with_prompt(prompt);
            }
            
            sel.interact()?
        };

        Ok(Some(list[index].clone()))
    }

    /// Create PackVersion selection description
    fn version_desc(version: &PackVersion) -> String {
        format!("{} - {} {}", version.id, version.version_type, version.name)
    }

}
