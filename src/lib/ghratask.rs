use std::{env::consts::EXE_SUFFIX, fs, io};

use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Status},
    version,
};
use crate::utils::{
    self,
    archive::{extract_tar_gz, extract_zip},
    fs::{mkdtemp, mkftemp, set_executable},
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
    pub fn sync(&mut self) -> task::Result {
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

    pub fn update(&mut self) -> task::Result {
        self.as_ghrtask().update()
    }

    fn as_ghrtask(&self) -> GHRTask<'a> {
        GHRTask {
            asset_filter: self.asset_filter,
            command: self.command,
            repo: self.repo,
            trim_version: self.trim_version,
            version_arg: self.version_arg,
        }
    }

    fn exists(&self) -> bool {
        self.as_ghrtask().exists()
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

        let bin_path = utils::env::home_dir()
            .join(".local")
            .join("bin")
            .join(format!("{}{}", &self.command, EXE_SUFFIX));

        let archive_path = mkftemp()?;
        github::download_release_asset(&asset, &archive_path);

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
        fs::copy(
            &extract_path.join(format!("{}{}", &self.command, EXE_SUFFIX)),
            &bin_path,
        )?;
        set_executable(&bin_path)?;
        fs::remove_dir_all(&extract_path)?;

        Ok(())
    }

    fn latest_release(&self) -> Result<Release, github::GitHubError> {
        self.as_ghrtask().latest_release()
    }
}
