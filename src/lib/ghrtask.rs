use std::io;

use utils;
use utils::github::{self, Asset, Release};

// GHRTask simplifies tasks that install from GitHub Releases.
pub struct GHRTask<'a> {
    pub asset_filter: fn(&Asset) -> bool,
    pub command: &'a str,
    pub repo: (&'a str, &'a str),
    pub trim_version: fn(String) -> String,
    pub version_arg: &'a str,
}

impl<'a> GHRTask<'a> {
    pub fn current_version(&self) -> String {
        let stdout = match utils::process::command_output(&self.command, &[self.version_arg]) {
            Ok(o) => String::from_utf8(o.stdout).unwrap_or_default(),
            Err(error) => {
                println!(
                    "error: `{} {}`: {}",
                    &self.command, &self.version_arg, error
                );
                String::new()
            }
        };
        let trimmed = (self.trim_version)(String::from(stdout));
        if trimmed.len() == 0 {
            String::from("unexpected")
        } else {
            String::from(trimmed)
        }
    }

    pub fn exists(&self) -> bool {
        match utils::process::command_output(&self.command, &[self.version_arg]) {
            Ok(output) => output.status.success(),
            Err(_error) => false,
        }
    }

    pub fn latest_release(&self) -> Result<Release, github::GitHubError> {
        github::latest_release(String::from(self.repo.0), String::from(self.repo.1))
    }

    pub fn sync(&self) -> io::Result<()> {
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

    pub fn update(&self) -> io::Result<()> {
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
        github::download_release_asset(&asset, &bin_path);

        Ok(())
    }
}
