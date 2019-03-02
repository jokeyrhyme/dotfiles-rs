use std::{
    fs, io,
    path::{Path, PathBuf},
};

use dirs;
use regex;
use serde_derive::{Deserialize, Serialize};
use toml;

use crate::{
    lib::task::{self, Status},
    utils::{
        archive::extract_zip_pattern,
        fs::{delete_if_exists, mkftemp},
        github,
    },
};

const JRDF_FILE: &str = "jokeyrhyme-dotfiles.toml";

// GhraFont simplifies tasks that install fonts from GitHub Release archives.
pub struct GhraFont<'a> {
    pub asset_re: &'a str,
    pub font_suffix: &'a str,     // e.g. ".otf" or ".ttf"
    pub repo: (&'a str, &'a str), // (owner, project)
}

impl<'a> GhraFont<'a> {
    pub fn sync(&self) -> task::Result {
        let font_dir = match dirs::font_dir() {
            Some(d) => d,
            None => return Ok(Status::Skipped),
        };
        let install_dir = font_dir.join(&self.repo.1);

        let jrdf_file = install_dir.join(JRDF_FILE);
        if let Ok(m) = JrdfMetadata::read(&jrdf_file) {
            return Ok(Status::NoChange(m.version));
        }

        let release = github::latest_release(self.repo.0, self.repo.1)?;
        self.install(&release)
    }

    pub fn update(&self) -> task::Result {
        let font_dir = match dirs::font_dir() {
            Some(d) => d,
            None => return Ok(Status::Skipped),
        };
        let install_dir = font_dir.join(&self.repo.1);

        let jrdf_file = install_dir.join(JRDF_FILE);
        let jrdf = match JrdfMetadata::read(&jrdf_file) {
            Ok(m) => m,
            Err(_) => return Ok(Status::Skipped),
        };

        let release = github::latest_release(self.repo.0, self.repo.1)?;
        if release.tag_name == jrdf.version {
            return Ok(Status::NoChange(jrdf.version));
        }

        match self.install(&release)? {
            Status::Changed(_, latest) => Ok(Status::Changed(jrdf.version, latest)),
            _ => Ok(Status::Changed(jrdf.version, String::from("unknown"))),
        }
    }

    fn install(&self, release: &github::Release) -> task::Result {
        let font_dir = match dirs::font_dir() {
            Some(d) => d,
            None => return Ok(Status::Skipped),
        };
        let install_dir = font_dir.join(&self.repo.1);

        let asset_re = regex::Regex::new(&self.asset_re).unwrap();
        let asset = github::compatible_asset(&release, &|a: &github::Asset| {
            asset_re.is_match(a.name.as_str())
        })?;
        let archive_path = mkftemp()?;
        github::download_release_asset(&asset, &archive_path)?;

        delete_if_exists(&install_dir);
        extract_zip_pattern(&archive_path, &install_dir, &|n| {
            n.to_lowercase().ends_with(&self.font_suffix)
        })?;

        fs::remove_file(&archive_path)?;

        let jrdf_file = install_dir.join(JRDF_FILE);
        let jrdf = JrdfMetadata {
            name: String::from(self.repo.1),
            version: release.tag_name.clone(),
        };
        jrdf.write(jrdf_file)?;

        Ok(Status::Changed(
            String::from("absent"),
            release.tag_name.clone(),
        ))
    }
}

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
