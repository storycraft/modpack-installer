/*
 * Created on Tue May 11 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use futures::{Future, Stream, StreamExt};
use tokio::fs;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{api::modpack::{data::PackFile, download_stream::PackStreamItem}, util::file::check_file};

pub struct FileInstalled {
    pub file: File,
    pub status: FileInstallStatus,
}

#[derive(Debug)]
pub enum FileInstallStatus {
    /// Valid file existed so skipped
    ValidFileExists,

    /// Successfully installed
    Installed,
}

#[derive(Debug)]
pub enum FileInstallError {
    Reqwest(reqwest::Error),
    Io(io::Error),
}

impl Display for FileInstallError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            FileInstallError::Reqwest(err) => err.fmt(f),
            FileInstallError::Io(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for FileInstallError {
    fn from(err: io::Error) -> Self {
        FileInstallError::Io(err)
    }
}

impl From<reqwest::Error> for FileInstallError {
    fn from(err: reqwest::Error) -> Self {
        FileInstallError::Reqwest(err)
    }
}

impl Error for FileInstallError {}

/// Pack install stream that uses PackDownloadStream
pub struct WebInstallStream<S> {
    /// Download stream
    stream: S,

    /// Download location
    location: PathBuf,
}

impl<S> WebInstallStream<S> {
    pub fn new(stream: S, location: PathBuf) -> Self {
        Self { stream, location }
    }
}

impl<S: Stream<Item = PackStreamItem> + Unpin> Stream for WebInstallStream<S> {
    type Item =
        Box<dyn Future<Output = (PackFile, Result<FileInstalled, FileInstallError>)> + Unpin + Send>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some((file, response_fut))) => {
                let location = self.location.clone();
                let file_dir = location.join(&file.info.path);
                let full_path = file_dir.join(&file.info.name);
                let should_download = !check_file(&full_path, file.info.size, &file.info.sha1);

                let fut = async move {
                    // Move request future so it can be dropped when it didnt used.
                    let response_fut = response_fut;

                    if should_download {
                        let mut res = response_fut.await?;
                        fs::create_dir_all(file_dir).await?;

                        let mut out_file = File::create(full_path)?;
                        {
                            let mut writer = BufWriter::new(&mut out_file);

                            while let Some(chunk) = res.chunk().await? {
                                writer.write_all(&chunk)?;
                            }
                            writer.flush()?;
                        }

                        Ok(FileInstalled { status: FileInstallStatus::Installed, file: out_file })
                    } else {
                        let file = File::open(full_path)?;
                        Ok(FileInstalled { status: FileInstallStatus::ValidFileExists, file })
                    }
                };

                Poll::Ready(Some(Box::new(Box::pin(async { (file, fut.await) }))))
            }

            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
