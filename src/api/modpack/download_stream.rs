/*
 * Created on Sat May 15 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{
    Future, Stream,
};
use reqwest::{Client, Response};

use super::data::PackFile;

/// Modpack file download stream
pub struct PackDownloadStream {
    client: Client,
    files: Vec<PackFile>,
}

impl PackDownloadStream {
    /// Create new downloader
    pub fn new(files: Vec<PackFile>) -> Self {
        Self::new_client(Client::new(), files)
    }

    /// Create downloader with custom client
    pub fn new_client(client: Client, files: Vec<PackFile>) -> Self {
        let mut files = files;
        files.reverse();

        Self { client, files }
    }
}

pub type PackStreamItem = (PackFile, Box<dyn Future<Output = Result<Response, reqwest::Error>> + Unpin + Send>);

impl Stream for PackDownloadStream {
    type Item = PackStreamItem;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
        match self.files.pop() {
            Some(file) => {
                let client = self.client.clone();
                let url = file.info.url.clone();

                let fut = Box::new(client.get(&url).send());

                Poll::Ready(Some((file, fut)))
            },
            None => Poll::Ready(None),
        }
    }
}
