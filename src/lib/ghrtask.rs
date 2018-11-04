use std::io;

use lib::{
    task::{self, Status},
    version,
};
use utils::{
    self,
    github::{self, Asset, Release},
};

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
        let trimmed = (self.trim_version)(stdout);
        if trimmed.is_empty() {
            String::from("unexpected")
        } else {
            trimmed
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

    pub fn sync(&self) -> task::Result {
        if self.exists() {
            return Ok(Status::Skipped);
        }

        let release = match self.latest_release() {
            Ok(r) => r,
            Err(error) => {
                return Err(task::Error::GitHubError(
                    format!("unable to check latest release for {:?}", &self.repo),
                    error,
                ));
            }
        };
        self.install_release(&release)?;
        Ok(Status::Changed("absent".to_string(), release.tag_name))
    }

    pub fn update(&self) -> task::Result {
        if !self.exists() {
            return Ok(Status::Skipped);
        }

        let current = self.current_version();
        if let Some(r) = github::release_versus_current(current.as_ref(), self.repo.0, self.repo.1)
        {
            self.install_release(&r)?;
            return Ok(Status::Changed(current, r.tag_name.to_string()));
        };
        Ok(Status::NoChange(current))
    }

    fn install_release(&self, release: &Release) -> io::Result<()> {
        let asset_filter = &self.asset_filter;
        let asset = match release
            .assets
            .to_vec()
            .into_iter()
            .find(|a| asset_filter(a) && version::is_stable(a.name.as_str()))
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
