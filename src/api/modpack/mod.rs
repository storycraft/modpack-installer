/*
 * Created on Fri May 07 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

//! FTB modpack API type declarations

pub mod info;
pub mod data;
pub mod search;
pub mod download_stream;

use reqwest::get;
use serde::{Deserialize, Serialize};

use crate::api::modpack::{data::PackVersionData, info::ModPack, search::SearchResult};

/// Modpack spec requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackSpec {
    /// Spec id(?)
    pub id: i32,

    /// Minimum ram size required (MB)
    pub minimum: u32,

    /// Recommended ram size required (MB)
    pub recommended: u32,
}

impl Default for PackSpec {
    fn default() -> Self {
        Self {
            id: 0,
            minimum: 4092,
            recommended: 6144
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ResSpecOption {

    Some(PackSpec),
    NoneEmptyStr(String)

}

pub fn empty_str_spec_as_none<'de, D>(de: D) -> Result<Option<PackSpec>, D::Error>
where
    D: serde::Deserializer<'de>
{
    let res = ResSpecOption::deserialize(de)?;

    match res {
        ResSpecOption::Some(spec) => Ok(Some(spec)),
        ResSpecOption::NoneEmptyStr(_) => Ok(None)
    }
}

/// Modpack addtional link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackLink {
    /// Link id(?)
    pub id: u32,

    /// Link name
    pub name: String,

    /// Link type (video, ...)
    #[serde(rename = "type")]
    pub link_type: String,

    /// Link url
    pub link: String,
}

/// Modpack tag data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackTag {
    /// Tag type id
    pub id: u32,

    /// Tag name
    pub name: String,
}

/// modpack.ch api endpoint
pub const API_URL: &'static str = "https://api.modpacks.ch";

/// Returns api endpoint
#[inline]
pub fn api_endpoint(path: &str) -> String {
    format!("{}/{}", API_URL, path)
}

pub type APIResult<T> = Result<T, reqwest::Error>;

pub struct ModPackAPI;

impl ModPackAPI {

    /// Search modpacks with limit
    pub async fn search(term: &str, limit: u32) -> APIResult<SearchResult> {
        let res = get(api_endpoint(&format!("public/modpack/search/{}?term={}", limit, term))).await?;

        res.json::<SearchResult>().await
    }

    /// Get modpack manifest using pack id
    pub async fn modpack_manifest(pack_id: u32) -> APIResult<ModPack> {
        let res = get(api_endpoint(&format!("public/modpack/{}", pack_id))).await?;

        res.json::<ModPack>().await
    }

    /// Get modpack version data using pack id and version id
    pub async fn modpack_version_data(pack_id: u32, version_id: u32) -> APIResult<PackVersionData> {
        let res = get(api_endpoint(&format!("public/modpack/{}/{}", pack_id, version_id))).await?;

        res.json::<PackVersionData>().await
    }

    /// Get curseforge modpack manifest using pack id
    pub async fn curseforge_manifest(pack_id: u32) -> APIResult<ModPack> {
        let res = get(api_endpoint(&format!("public/curseforge/{}", pack_id))).await?;

        res.json::<ModPack>().await
    }

    /// Get curseforge modpack version data using pack id and version id
    pub async fn curseforge_version_data(pack_id: u32, version_id: u32) -> APIResult<PackVersionData> {
        let res = get(api_endpoint(&format!("public/curseforge/{}/{}", pack_id, version_id))).await?;

        res.json::<PackVersionData>().await
    }

}
