/*
 * Created on Wed May 12 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::io::{self, BufWriter, Read};
use sha1::{Sha1, Digest};

/// Check if the reader data is valid using given sha1 hex hash.
pub fn validate_data(hash: &str, reader: &mut impl Read) -> bool {
    let mut hash_bytes = [0u8; 20];

    let decode_res = hex::decode_to_slice(&hash, &mut hash_bytes);
    if decode_res.is_err() {
        return false;
    }

    let mut hasher = Sha1::new();

    let copy_res = io::copy(reader, &mut BufWriter::new(&mut hasher));

    match copy_res {
        Ok(_) => {
            let hash = hasher.finalize().to_vec();

            if hash.eq(&hash_bytes) {
                true
            } else {
                false
            }
        },

        Err(_) => false
    }
}
