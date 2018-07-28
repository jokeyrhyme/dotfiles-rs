use std::{fs, io};

use lib::ghrtask::GHRTask;
use utils::{
    self, archive::{extract_tar_gz, extract_zip}, fs::{mkdtemp, mktemp},
    github::{self, Asset, Release},
};

// GHRATask simplifies tasks that install from GitHub Release archives.
pub struct GHRATask<'a> {
    pub asset_filter: fn(&Asset) -> bool,
    pub command: &'a str,
    pub repo: (&'a str, &'a str),
    pub trim_version: fn(String) -> String,
    pub version_arg: &'a str,
}

impl<'a> GHRATask<'a> {
    pub fn sync(&mut self) -> io::Result<()> {
        if self.exists() {
            return Ok(());
        }
        println!("{}: syncing...", &self.command);

        let release = match self.latest_release() {
            Ok(r) => r,
            Err(error) => {
                println!(
                    "error: unable to check latest release: {:?} {:?}",
                    &self.repo, error
                );
                return Ok(());
            }
        };
        self.install_release(&release)?;
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        if !self.exists() {
            return Ok(());
        }
        println!("{}: updating...", &self.command);

        let current = self.current_version();
        match github::release_versus_current(
            current,
            String::from(self.repo.0),
            String::from(self.repo.1),
        ) {
            Some(r) => self.install_release(&r)?,
            None => {}
        };
        Ok(())
    }

    fn as_ghrtask(&self) -> GHRTask<'a> {
        GHRTask {
            asset_filter: self.asset_filter.clone(),
            command: self.command,
            repo: self.repo,
            trim_version: self.trim_version.clone(),
            version_arg: self.version_arg,
        }
    }

    fn current_version(&self) -> String {
        self.as_ghrtask().current_version()
    }

    fn exists(&self) -> bool {
        self.as_ghrtask().exists()
    }

    fn install_release(&self, release: &Release) -> io::Result<()> {
        let asset = match release
            .assets
            .to_vec()
            .into_iter()
            .filter(self.asset_filter)
            .next()
        {
            Some(a) => a,
            None => {
                println!("warning: {}: no asset matches OS and ARCH", &self.command);
                return Ok(());
            }
        };
        println!("{}: installing...", &self.command);

        let bin_path = utils::env::home_dir()
            .join(".local")
            .join("bin")
            .join(&self.command);

        let archive_path = mktemp()?;
        github::download_release_asset(&asset, &archive_path);

        println!("{}: extracting...", &self.command);
        let extract_path = mkdtemp()?;
        if asset.name.ends_with(".tar.gz") {
            extract_tar_gz(&archive_path, &extract_path)?;
        } else if asset.name.ends_with(".zip") {
            extract_zip(&archive_path, &extract_path)?;
        } else {
            let msg = format!("unexpected archive file type: {}", &asset.name);
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        }

        fs::remove_file(&archive_path)?;

        println!(
            "{}: copying: {} -> {}",
            &self.command,
            &extract_path.join(&self.command).display(),
            &bin_path.display(),
        );
        fs::copy(&extract_path.join(&self.command), &bin_path)?;

        fs::remove_dir_all(&extract_path)?;

        Ok(())
    }

    fn latest_release(&self) -> Result<Release, github::GitHubError> {
        self.as_ghrtask().latest_release()
    }
}
