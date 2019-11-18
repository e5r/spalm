use log::{error, trace};
use std::io::Write;
use std::path::PathBuf;

use crate::cli::Cmd;

pub struct InitCommand {
    path: Option<PathBuf>,
}

impl InitCommand {
    pub fn new(args: Vec<String>) -> InitCommand {
        match args.first() {
            Some(dir) => InitCommand {
                path: Some(PathBuf::from(dir.to_string())),
            },
            _ => InitCommand { path: None },
        }
    }

    pub fn usage() {
        let exe_name = crate::utils::get_exe_name();

        println!("Usage: {} init [DIRECTORY]", exe_name);
    }
}

impl Cmd for InitCommand {
    fn exec(self) -> Result<(), (i32)> {
        let path = match self.path {
            Some(path) => path,
            None => std::env::current_dir().unwrap(),
        };

        if !path.exists() {
            if let Err(_) = std::fs::create_dir_all(&path) {
                error!("Error on create directory \"{}\".", &path.display());
                return Err(3);
            }
        }

        if !path.is_dir() {
            error!("The path must be a directory.");
            return Err(4);
        }

        let mut entries = vec![];
        {
            entries.push(PathEntry::new_dir(path.join("doc")));
            entries.push(PathEntry::new_dir(path.join("doc").join("spec")));
            entries.push(PathEntry::new_dir(
                path.join("doc").join("spec").join("proposal"),
            ));
            entries.push(PathEntry::new_empty_file(
                path.join("doc")
                    .join("spec")
                    .join("proposal")
                    .join(".empty"),
            ));
            entries.push(PathEntry::new_dir(
                path.join("doc").join("spec").join("refused"),
            ));
            entries.push(PathEntry::new_empty_file(
                path.join("doc").join("spec").join("refused").join(".empty"),
            ));
            entries.push(PathEntry::new_dir(
                path.join("doc").join("spec").join("specification"),
            ));
            entries.push(PathEntry::new_empty_file(
                path.join("doc")
                    .join("spec")
                    .join("specification")
                    .join(".empty"),
            ));
            entries.push(PathEntry::new_dir(path.join("doc").join("tutorials")));
            entries.push(PathEntry::new_empty_file(
                path.join("doc").join("tutorials").join(".empty"),
            ));
            entries.push(PathEntry::new_empty_file(path.join("doc").join("index.md")));
            entries.push(PathEntry::new_empty_file(
                path.join("doc").join("project.md"),
            ));
        }

        entries.sort_by(|a, b| a.is_file.cmp(&b.is_file));

        for e in entries {
            let empty_content: Vec<u8> = vec![];
            let entry_type = if e.is_file { "file" } else { "directory" };

            if e.path.exists() {
                trace!(
                    "Skipping {} \"{}\" because it already exists.",
                    &entry_type,
                    &e.path.display()
                );
                continue;
            } else if e.is_file {
                trace!("Creating {} \"{}\"", &entry_type, &e.path.display());
                let mut file = std::fs::File::create(e.path).unwrap();
                file.write_all(e.file_content.unwrap_or(empty_content).as_ref())
                    .unwrap();
            } else {
                trace!("Creating {} \"{}\"", &entry_type, &e.path.display());
                std::fs::create_dir_all(&e.path).unwrap();
            }
        }

        println!(
            "The E5R Specification Project was successfully initialized to \"{}\".",
            get_full_path(&path)
        );

        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn get_full_path(path: &PathBuf) -> String {
    path.canonicalize().unwrap().display().to_string()
}

#[cfg(target_os = "windows")]
fn get_full_path(path: &PathBuf) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let path = path.canonicalize().unwrap().display().to_string();
    if path.starts_with(VERBATIM_PREFIX) {
        path[VERBATIM_PREFIX.len()..].to_string()
    } else {
        path
    }
}

struct PathEntry {
    pub path: PathBuf,
    pub is_file: bool,
    pub file_content: Option<Vec<u8>>,
}

impl PathEntry {
    fn new_dir(path: PathBuf) -> Self {
        PathEntry {
            path,
            is_file: false,
            file_content: None,
        }
    }

    fn new_empty_file(path: PathBuf) -> Self {
        PathEntry {
            path,
            is_file: true,
            file_content: Some(
                "Remove this file after add you first file"
                    .as_bytes()
                    .to_vec(),
            ),
        }
    }
}
