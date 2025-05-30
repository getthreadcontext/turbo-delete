/*
  Copyright 2022 Tejas Ravishankar

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

use owo_colors::{AnsiColors, OwoColorize};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

// change a file to be writable
pub fn set_writable(path: &Path) {
    if let Ok(metadata) = std::fs::metadata(path) {
        let mut perms = metadata.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            let _ = std::fs::set_permissions(path, perms);
        }
    }
}

pub fn force_delete_entry(path: &Path) {
    if path.is_file() {
        if std::fs::remove_file(path).is_err() {
            set_writable(path);
            let _ = std::fs::remove_file(path);
        }
    } else if path.is_dir() {
        if std::fs::remove_dir_all(path).is_err() {
            // Try to make writable and delete again
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    set_writable(&entry.path());
                }
            }
            let _ = std::fs::remove_dir_all(path);
        }
    }
}

fn main() {
    // Display Turbo Delete header
    println!("{}", "╔══════════════════════════════════════╗".bright_cyan());
    println!("{}", "║        🚀 Turbo Delete v1.0.0        ║".bright_cyan());
    println!("{}", "║    Fast & Parallel File Deletion    ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════╝".bright_cyan());
    println!();

    let start = Instant::now();

    let args = std::env::args().collect::<Vec<String>>();

    let mut file_path: String = args
        .get(1)
        .unwrap_or_else(|| {
            eprintln!(
                "{} {}\n\n{}:\n{} {}",
                " ERROR ".on_color(AnsiColors::BrightRed).black(),
                "Please provide a folder path.".bright_yellow(),
                "Examples".underline(),
                "turbodelete".bright_cyan(),
                "./node_modules/".bright_black(),
            );
            std::process::exit(1);
        })
        .to_string();

    if file_path.ends_with('"') {
        file_path.pop();
    }

    let path = PathBuf::from(&file_path);    if !path.exists() {
        println!("{} {}", 
            " INFO ".on_color(AnsiColors::BrightBlue).white(),
            "Path does not exist or already deleted.".bright_yellow()
        );
        return;
    }

    println!("{} {}", 
        " TARGET ".on_color(AnsiColors::BrightGreen).black(),
        format!("Preparing to delete: {}", file_path).bright_white()
    );
    
    println!("{} {}", 
        " SCAN ".on_color(AnsiColors::BrightYellow).black(),
        "Scanning directory structure...".bright_white()
    );

    // Collect all entries first, then delete in parallel
    let entries: Vec<PathBuf> = jwalk::WalkDir::new(&path)
        .follow_links(false)
        .skip_hidden(false)
        .sort(true)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    println!("{} {}", 
        " FOUND ".on_color(AnsiColors::BrightMagenta).white(),
        format!("Discovered {} items to delete", entries.len()).bright_white()
    );
    
    println!("{} {}", 
        " DELETE ".on_color(AnsiColors::BrightRed).white(),
        "Deleting files and directories in parallel...".bright_white()
    );

    // Delete all entries in parallel
    entries.par_iter().for_each(|entry_path| {
        force_delete_entry(entry_path);
    });    // Final cleanup - delete the root directory
    if path.exists() {
        force_delete_entry(&path);
    }

    let elapsed = start.elapsed().as_secs_f32();
    
    println!();
    println!("{} {}", 
        " SUCCESS ".on_color(AnsiColors::BrightGreen).black(),
        "Deletion completed successfully!".bright_white()
    );
    
    println!("{} {}", 
        " TIME ".on_color(AnsiColors::BrightBlue).white(),
        format!("Completed in {:.3}s", elapsed).bright_white()
    );
    
    if elapsed < 1.0 {
        println!("{} {}", 
            " SPEED ".on_color(AnsiColors::BrightCyan).black(),
            "⚡ Lightning fast deletion!".bright_yellow()
        );
    } else if elapsed < 5.0 {
        println!("{} {}", 
            " SPEED ".on_color(AnsiColors::BrightCyan).black(),
            "🚀 Turbo speed achieved!".bright_yellow()
        );
    }
    
    println!();
}

