/*
 * Created on Tue May 11 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Seek},
    path::PathBuf,
};

use zip::{result::ZipError, ZipArchive};

use crate::api::overrides::manifest::PackManifest;

// Package install errors
#[derive(Debug)]
pub enum PackageInstallError {
    Package(ZipError),
    Io(io::Error),
    Manifest(serde_json::Error),
}

impl From<ZipError> for PackageInstallError {
    fn from(err: ZipError) -> Self {
        PackageInstallError::Package(err)
    }
}

impl From<io::Error> for PackageInstallError {
    fn from(err: io::Error) -> Self {
        PackageInstallError::Io(err)
    }
}

impl From<serde_json::Error> for PackageInstallError {
    fn from(err: serde_json::Error) -> Self {
        PackageInstallError::Manifest(err)
    }
}

/// Pack installer that uses package zip (known as overrides.zip)
pub struct PackageInstaller<T: Read + Seek> {
    /// Package reader
    archive: ZipArchive<T>,
}

impl<T: Read + Seek> PackageInstaller<T> {
    pub fn new(archive: ZipArchive<T>) -> Self {
        Self { archive }
    }

    /// Read package and install to location
    pub async fn install(mut self, location: PathBuf) -> Result<(), PackageInstallError> {
        let manifest = {
            let file = self.archive.by_name("manifest.json")?;

            Ok::<PackManifest, PackageInstallError>(serde_json::from_reader(file)?)
        }?;

        match &manifest.overrides {
            Some(override_dir) => {
                let override_dir_path = PathBuf::from(&override_dir);
                let override_list: Vec<String> = self
                    .archive
                    .file_names()
                    .filter(|path| path.starts_with(override_dir)) // unwrap
                    .map(|path| path.clone().into())
                    .collect();

                for override_path in &override_list {
                    let out_path = {
                        let path: PathBuf = override_path.into();

                        path.into_iter().skip(override_dir_path.iter().count()).collect::<PathBuf>() // todo
                    };

                    let mut reader = BufReader::new(self.archive.by_name(override_path)?);
                    let mut writer = BufWriter::new(File::create(location.join(out_path))?);

                    io::copy(&mut reader, &mut writer)?;
                }
            }
            None => {}
        };

        for file in &manifest.files {
            // TODO
        }

        Ok(())
    }
}
