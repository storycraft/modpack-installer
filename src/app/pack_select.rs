/*
 * Created on Sun May 09 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use dialoguer::Select;
use futures::{stream, StreamExt};
use std::{error::Error, fmt::{Display, Formatter}, io};
use tokio::task::JoinError;

use crate::api::modpack::{APIResult, ModPackAPI, info::ModPack, search::SearchResult};

/// Modpack select screen
pub struct ModpackSelect {
    list: Vec<ModPackVariant>,
}

impl ModpackSelect {
    /// Create new Selection
    pub fn new(list: Vec<ModPackVariant>) -> Self {
        Self { list }
    }

    /// Display selection
    pub fn select(self, prompt: Option<&str>) -> Result<Option<ModPackVariant>, io::Error> {
        let list = self.list;

        if list.len() < 1 {
            return Ok(None);
        }

        let index = {
            let mut sel = Select::new();
            sel.items(&list.iter().map(|item| item.display()).collect::<Vec<String>>()).paged(true).default(0);

            if let Some(prompt) = prompt {
                sel.with_prompt(prompt);
            }
            
            sel.interact()?
        };

        Ok(Some(list[index].clone()))
    }
}

#[derive(Debug, Clone)]
pub enum ModPackVariant {
    ModPack(ModPack),
    CurseForge(ModPack),
}

impl ModPackVariant {

    /// Inner modpack struct
    pub fn info(&self) -> &ModPack {
        match &self {
            ModPackVariant::ModPack(pack) => pack,
            ModPackVariant::CurseForge(pack) => pack,
        }
    }

    /// Display string
    pub fn display(&self) -> String {
        let mut str = String::new();

        match &self {
            ModPackVariant::CurseForge(_) => str.push_str("(curseforge) "),
            
            _ => {}
        }

        let info = self.info();

        str.push_str(&format!("{} - {}", info.id, &console::style(&info.name).green().to_string()));

        let authors = info.author_str();

        if authors.len() > 1 {
            str.push_str(&format!(" by {}", authors));
        }

        str
    }

}

#[derive(Debug)]
pub enum TaskError {
    Thread(JoinError),
    Reqwest(reqwest::Error),
}

impl From<JoinError> for TaskError {
    fn from(err: JoinError) -> Self {
        Self::Thread(err)
    }
}

impl From<reqwest::Error> for TaskError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl Error for TaskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            TaskError::Thread(err) => Some(err),
            TaskError::Reqwest(err) => Some(err)
        }
    }
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            TaskError::Thread(err) => err.fmt(f),
            TaskError::Reqwest(err) => err.fmt(f)
        }
    }
}

/// Create ModPackVariant list from search result
pub async fn create_list_from_result(
    result: SearchResult,
) -> Result<Vec<ModPackVariant>, TaskError> {
    let mut list: Vec<ModPackVariant> =
        Vec::with_capacity(result.packs.len() + result.curseforge.len());

    let modpack_stream = stream::iter(result.packs).map(ModPackAPI::modpack_manifest);
    let curseforge_stream = stream::iter(result.curseforge).map(ModPackAPI::curseforge_manifest);

    let modpack_task = tokio::spawn(
        modpack_stream
            .buffer_unordered(16)
            .collect::<Vec<APIResult<ModPack>>>(),
    );
    let curseforge_task = tokio::spawn(
        curseforge_stream
            .buffer_unordered(16)
            .collect::<Vec<APIResult<ModPack>>>(),
    );

    let (modpack_res_list, curseforge_res_list) = (modpack_task.await?, curseforge_task.await?);

    for modpack_res in modpack_res_list {
        list.push(ModPackVariant::ModPack(modpack_res?));
    }

    for curseforge_res in curseforge_res_list {
        list.push(ModPackVariant::CurseForge(curseforge_res?));
    }

    Ok(list)
}
