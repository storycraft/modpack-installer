/*
 * Created on Mon May 17 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::{io::BufReader, path::PathBuf, fs};

use super::hash::validate_data;

/// Check if file is valid
pub fn check_file(path: &PathBuf, size: i64, sha1: &str) -> bool {
    if let Ok(meta) = fs::metadata(&path) {
        if !meta.is_file() || size < 0 || meta.len() != size as u64 {
            false
        } else {
            if let Ok(reader) = fs::File::open(&path) {
                let mut reader = BufReader::new(reader);

                validate_data(sha1, &mut reader)
            } else {
                false
            }
        }
    } else {
        false
    }
}
