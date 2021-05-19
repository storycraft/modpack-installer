/*
 * Created on Tue May 18 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::{
    io::BufReader,
    path::{Path, PathBuf}
};

use futures::StreamExt;
use humansize::{file_size_opts, FileSize};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::task::JoinHandle;
use zip::ZipArchive;

use crate::{
    api::modpack::{
        data::{PackFile, PackFileType},
        download_stream::PackDownloadStream,
    },
    app::{
        pack_install::{
            package::PackageInstaller,
            web::{FileInstallStatus, WebInstallStream},
        },
        AppError,
    },
};

/// Spawn pack install task using files and install location.
pub fn spawn_install_task(
    files: Vec<PackFile>,
    install_location: PathBuf,
    multi: &MultiProgress,
) -> JoinHandle<Result<(), AppError>> {
    let total = multi.add(ProgressBar::new(files.len() as u64));
    tokio::spawn(async move {
        total.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/gray}] {pos} / {len} ({eta})",
        ));

        let mut stream = WebInstallStream::new(PackDownloadStream::new(files), install_location)
            .buffer_unordered(60);

        while let Some((file, res)) = stream.next().await {
            match res {
                Ok(result) => {
                    match &result.status {
                        FileInstallStatus::ValidFileExists => {
                            total.println(
                                &console::style(format!(
                                    "{} already installed. Skipping...",
                                    &file.info.name
                                ))
                                .green()
                                .to_string(),
                            );
                        }

                        FileInstallStatus::Installed => {}
                    }

                    // Handle overrides.zip
                    if let PackFileType::Overrides = &file.file_type {
                        let archive = ZipArchive::new(BufReader::new(result.file))?;

                        let installer = PackageInstaller::new(archive);

                        total.println("[warn] Package install is not yet implemented");
                    }

                    total.println(format_file_info(&file));
                }

                Err(err) => {
                    total.println(
                        &console::style(format!(
                            "Error occured while downloading {}. err: {}",
                            &file.info.name, err
                        ))
                        .red()
                        .to_string(),
                    );
                }
            }

            total.inc(1);
        }

        total.finish();

        Ok(())
    })
}

fn format_file_info(file: &PackFile) -> String {
    let prefix = match &file.file_type {
        PackFileType::Mod => {
            format!("[{}]", console::style("mod").cyan())
        }
        PackFileType::Resource => {
            format!("[{}]", console::style("resource").magenta())
        }
        PackFileType::Config => {
            format!("[{}]", console::style("config").green())
        }
        PackFileType::Script => {
            format!("[{}]", console::style("script").yellow())
        }
        PackFileType::Overrides => {
            format!("[{}]", console::style("package").blue())
        }
    };

    let info = &file.info;

    let full_name = {
        let path = Path::new(&info.path);
        let full_path = path.join(&info.name);

        String::from(full_path.to_string_lossy())
    };

    format!(
        "{} {} - {}",
        prefix,
        full_name,
        if info.size > 0 {
            info.size.file_size(file_size_opts::BINARY).unwrap()
        } else {
            "unknown".into()
        }
    )
}
