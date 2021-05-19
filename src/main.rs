/*
 * Created on Fri May 07 2021
 *
 * Copyright (c) storycraft. Licensed under the GNU General Public License v3.
 */

use std::time::SystemTime;

pub mod api;
pub mod launcher;
pub mod app;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = SystemTime::now();

    let args = std::env::args().map(|arg| arg.to_string()).collect::<Vec<String>>();
    
    match args.len() {
        // Run default app
        1 => {
            match app::run().await {
                Ok(_) => {
                    let elapsed = start.elapsed()?;
        
                    println!(
                        "{}",
                        console::style(format!("Done. took {} ms", elapsed.as_millis())).green()
                    );
                },
        
                Err(err) => {
                    println!(
                        "{}",
                        console::style(format!("Error while processing. err: {}", err)).red()
                    );
                }
            }

            console::Term::stdout().read_key()?;
        }

        // Run package installer
        2 => {
            println!("Package installer is not implemented yet");
            todo!();
        }

        // Show description and helpmap
        _ => {
            println!("Usage: {} [modpack zip to install]", args.get(0).unwrap_or(&"modpack-installer".into()));
        }
    }

    Ok(())
}
