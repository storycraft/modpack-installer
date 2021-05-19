/*
 * Created on Sun May 09 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

pub mod mc_data_dir_select;
pub mod pack_install;
pub mod pack_select;
pub mod tasks;
pub mod ver_select;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    path::PathBuf
};

use chrono::Utc;
use dialoguer::{Confirm, Input};
use indicatif::MultiProgress;
use tokio::fs;

use crate::{
    api::modpack::{
        data::{PackFile, PackVersionData},
        info::{ModPack, PackArt, PackArtType},
        search::SearchResult,
        ModPackAPI,
    },
    app::{tasks::install_pack::spawn_install_task, ver_select::PackVersionSelect},
    launcher::profile::{GameLaunchProfile, LauncherProfile},
};

use self::{
    mc_data_dir_select::MCDataDirSelect,
    pack_select::{create_list_from_result, ModPackVariant, ModpackSelect, TaskError},
};

/// App error
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Reqwest(reqwest::Error),
    Archive(zip::result::ZipError),
    Task(TaskError),
    Profile(serde_json::Error),
    InvalidPack,
    Cancelled,
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<TaskError> for AppError {
    fn from(err: TaskError) -> Self {
        Self::Task(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Profile(err)
    }
}

impl From<zip::result::ZipError> for AppError {
    fn from(err: zip::result::ZipError) -> Self {
        Self::Archive(err)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            AppError::Io(err) => err.fmt(f),
            AppError::Reqwest(err) => err.fmt(f),
            AppError::Task(err) => err.fmt(f),
            AppError::Profile(err) => err.fmt(f),
            AppError::Archive(err) => err.fmt(f),
            AppError::InvalidPack => writeln!(f, "Invalid pack to install"),
            AppError::Cancelled => writeln!(f, "Cancelled by user"),
        }
    }
}

impl Error for AppError {}

/// Main app
pub async fn run() -> Result<(), AppError> {
    // Select minecraft dir
    let (data_path, launcher_profile) = ask_minecraft_dir()?;

    // Ask pack
    let pack = ask_pack_to_install().await?;
    let info = pack.info();

    console::Term::stdout().clear_screen().unwrap_or_default();

    // Print pack info
    print_pack_info(info);

    let ver = ask_pack_version(&pack).await?;

    if ver.is_none() {
        return Err(AppError::InvalidPack);
    }

    let ver = ver.unwrap();

    console::Term::stdout().clear_screen().unwrap_or_default();

    let install_location = ask_install_location(data_path.join("modpacks").join(&info.name))?;

    print_install_info(info, &ver, &install_location);

    let files = install_files(&ver)?;

    let mut confirm = Confirm::new();
    confirm.with_prompt("Install?");
    if !confirm.interact()? {
        return Err(AppError::Cancelled);
    }

    let multi = MultiProgress::new();
    let install_task_handle = spawn_install_task(files, install_location.clone(), &multi);

    multi.join()?;
    if let Ok(res) = install_task_handle.await {
        res?;
    }

    println!("{}", console::style("Installing pack profile...").yellow());

    let game_profile = {
        let icon: String = create_profile_icon(&info).await;
        let modloader = ver
            .targets
            .iter()
            .filter(|target| target.target_type == "modloader")
            .next()
            .unwrap();

        let game = ver
            .targets
            .iter()
            .filter(|target| target.target_type == "game")
            .next()
            .unwrap();

        println!(
            "{} {}",
            console::style(format!("Install {} {} {} from", modloader.name, game.version, modloader.version)).yellow(),
            console::style(format!("https://files.minecraftforge.net/net/minecraftforge/forge/index_{}.html", game.version)).yellow().bold()
        );


        let time: String = Utc::now().to_string();

        GameLaunchProfile {
            created: Some(time.clone()),
            last_used: Some(time.clone()),
            game_dir: Some(install_location.to_string_lossy().into()),
            java_args: None,
            last_version_id: format!("change-this-to-valid-{}-{}-{}-version", game.version, modloader.name, modloader.version),
            icon: Some(icon),
            name: info.name.clone(),
            profile_type: "custom".into(),
            extra: Default::default(),
        }
    };

    let new_profile = {
        let mut new = launcher_profile.clone();

        new.profiles
            .insert(format!("modpack-{}", info.id), game_profile);

        new
    };

    fs::write(
        data_path.join("launcher_profiles.json"),
        serde_json::to_string_pretty(&new_profile)?,
    )
    .await?;

    println!("{}",console::style("Finished installing modpack. Adjust game profile manually for proper launch.").green());

    Ok(())
}

/// Ask minecraft data dir
fn ask_minecraft_dir() -> Result<(PathBuf, LauncherProfile), AppError> {
    let mc_input = MCDataDirSelect::new_default();
    Ok(mc_input.select(Some("Put Minecraft data directory location\n"))?)
}

/// Ask modpack to search and return selected pack
async fn ask_pack_to_install() -> Result<ModPackVariant, AppError> {
    let ask = || async {
        let pack_list = create_list_from_result(search_pack().await?).await?;
        let pack_selector = ModpackSelect::new(pack_list);

        Ok::<Option<ModPackVariant>, AppError>(
            pack_selector.select(Some("Select modpack to install using arrow key"))?,
        )
    };

    let mut selected = ask().await?;

    while let None = selected {
        let sel = ask().await?;

        if sel.is_some() {
            selected = sel;
        }

        println!("{}", console::style("Cannot find any modpacks").red());
    }

    Ok(selected.unwrap())
}

/// Ask search term to user and return search result
async fn search_pack() -> Result<SearchResult, AppError> {
    let mut input = Input::<String>::new();
    input.with_prompt("Modpack name to install\n");

    let keyword = input.interact_text()?;

    Ok::<SearchResult, AppError>(ModPackAPI::search(&keyword, 50).await?)
}

/// Print modpack information to terminal
fn print_pack_info(info: &ModPack) {
    let authors = info.author_str();

    let tags = info
        .tags
        .iter()
        .map(|tag| format!("#{}", tag.name.clone()))
        .collect::<Vec<String>>()
        .join(", ");

    println!(
        "{}",
        console::style(format!(
            "{} by {}",
            &console::style(&info.name).green(),
            authors
        ))
        .bold()
    );

    if let Some(desc) = info.description.as_ref() {
        println!("{}", desc);
    }
    println!("{}", console::style(&tags).yellow());
}

/// Ask pack version to install and fetch manifest
async fn ask_pack_version(pack: &ModPackVariant) -> Result<Option<PackVersionData>, AppError> {
    let ver_selector = PackVersionSelect::new(pack.info().versions.clone());
    let ver = ver_selector.select(Some("Select version to install using arrow key"))?;

    if ver.is_none() {
        return Ok(None);
    }
    let ver = ver.unwrap();

    println!("{}", console::style("Preparing version data...").yellow());

    let version_data = match pack {
        ModPackVariant::ModPack(info) => ModPackAPI::modpack_version_data(info.id, ver.id).await?,
        ModPackVariant::CurseForge(info) => {
            ModPackAPI::curseforge_version_data(info.id, ver.id).await?
        }
    };

    Ok(Some(version_data))
}

/// Ask install directory
fn ask_install_location(default_path: PathBuf) -> Result<PathBuf, AppError> {
    let mut input = Input::<String>::new();
    input.with_prompt("Input modpack install directory\n");
    input.with_initial_text(default_path.to_string_lossy());

    Ok(input.interact_text()?.into())
}

/// Print install information
fn print_install_info(pack: &ModPack, version: &PackVersionData, location: &PathBuf) {
    println!("name: {}", console::style(&pack.name).yellow());
    println!("type: {}", console::style(&version.version_type).yellow());
    println!("version: {}", console::style(&version.name).yellow());

    println!(
        "install location: {}",
        console::style(location.to_string_lossy()).yellow()
    );

    let spec = version.specs.clone().unwrap_or_default();

    println!(
        "RAM recommendation: {}",
        console::style(format!(
            "{} GB",
            (spec.recommended as f32 / 1024_f32).ceil()
        ))
        .yellow()
    );
}

// Get files to install from pack version
fn install_files(ver: &PackVersionData) -> Result<Vec<PackFile>, AppError> {
    let required = ver
        .files
        .clone()
        .into_iter()
        .filter(|file| !file.info.optional)
        .collect::<Vec<PackFile>>();

    if required.len() != ver.files.len() {
        let mut optional_ask = Confirm::new();
        optional_ask.with_prompt("Install optional resources?");
        if !optional_ask.interact()? {
            Ok(required)
        } else {
            Ok(ver.files.clone())
        }
    } else {
        Ok(required)
    }
}

/// Create base64 modpack profile icon
async fn create_profile_icon(pack: &ModPack) -> String {
    let icon_list = pack
        .arts
        .clone()
        .into_iter()
        .filter(|art| art.art_type == PackArtType::Square)
        .collect::<Vec<PackArt>>();

    if icon_list.len() < 1 {
        "Furnace".into()
    } else {
        let icon = &icon_list[0];

        (|| async {
            let res = reqwest::get(&icon.info.url).await?;

            let base64 = base64::encode(res.bytes().await?.to_vec());

            Ok::<String, reqwest::Error>(format!("data:image/png;base64,{}", base64))
        })()
        .await
        .unwrap_or("Furnace".into())
    }
}
