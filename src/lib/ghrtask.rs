use std::env::consts::EXE_SUFFIX;

use crate::lib::task::{self, Status};
use crate::utils::{
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
        Ok(Status::Changed(String::from("absent"), release.tag_name))
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

    fn install_release(&self, release: &Release) -> github::Result<()> {
        let asset = github::compatible_asset(&release, &self.asset_filter)?;

        let bin_path = utils::env::home_dir()
            .join(".local")
            .join("bin")
            .join(format!("{}{}", &self.command, EXE_SUFFIX));
        github::download_release_asset(&asset, &bin_path)?;

        Ok(())
    }
}
