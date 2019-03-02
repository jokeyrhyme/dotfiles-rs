use std::{
    fs, io,
    path::{Path, PathBuf},
};

use dirs;
use regex;
use serde_derive::{Deserialize, Serialize};
use toml;

use crate::{
    lib::task::{self, Status, Task},
    utils::{archive::extract_zip_pattern, fs::mkftemp, github},
};

const ASSET_RE: &str = r"^Inter-.*\.zip$";
const GH_REPO: &str = "inter";
const GH_OWNER: &str = "rsms";
const JRDF_FILE: &str = "jokeyrhyme-dotfiles.toml";

#[derive(Debug, Deserialize, Serialize)]
struct JrdfMetadata {
    pub name: String,
    pub version: String,
}
impl JrdfMetadata {
    fn read<P>(p: P) -> io::Result<JrdfMetadata>
    where
        P: Into<PathBuf> + AsRef<Path>,
    {
        let contents = fs::read_to_string(&p.as_ref())?;
        match toml::from_str(&contents) {
            Ok(t) => Ok(t),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }
    fn write<P>(&self, p: P) -> io::Result<()>
    where
        P: Into<PathBuf> + AsRef<Path>,
    {
        let contents = match toml::to_string_pretty(&self) {
            Ok(t) => t,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };
        fs::write(&p.as_ref(), contents)
    }
}

pub fn task() -> Task {
    Task {
        name: String::from("inter"),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    let font_dir = match dirs::font_dir() {
        Some(d) => d,
        None => return Ok(Status::Skipped),
    };
    let install_dir = font_dir.join("inter");
    let jrdf_file = install_dir.join(JRDF_FILE);

    if let Ok(m) = JrdfMetadata::read(&jrdf_file) {
        return Ok(Status::NoChange(m.version));
    }
    let release = github::latest_release(GH_OWNER, GH_REPO)?;
    let asset_re = regex::Regex::new(ASSET_RE).unwrap();
    let asset = github::compatible_asset(&release, &|a: &github::Asset| {
        asset_re.is_match(a.name.as_str())
    })?;
    let archive_path = mkftemp()?;
    github::download_release_asset(&asset, &archive_path)?;

    extract_zip_pattern(&archive_path, &install_dir, &|n| {
        n.to_lowercase().ends_with(".otf")
    })?;

    fs::remove_file(&archive_path)?;

    let jrdf = JrdfMetadata {
        name: String::from("inter"),
        version: release.tag_name.clone(),
    };
    jrdf.write(jrdf_file)?;

    Ok(Status::Changed(String::from("absent"), release.tag_name))
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
