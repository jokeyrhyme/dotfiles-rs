use std;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str;

use serde_json;

use utils;

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
    state: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub assets: Vec<Asset>,
    #[serde(default = "default_json_false")]
    draft: bool,
    name: String,
    #[serde(default = "default_json_false")]
    prelease: bool,
    pub tag_name: String,
}

fn default_json_false() -> bool {
    false
}

#[derive(Debug)]
pub struct EmptyReleasesError {}

impl Display for EmptyReleasesError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "EmptyReleasesError")
    }
}

impl std::error::Error for EmptyReleasesError {
    fn cause<'a>(&'a self) -> Option<&'a Error> {
        None
    }
    fn description<'a>(&'a self) -> &'a str {
        &"EmptyReleasesError"
    }
}

fn fetch_releases<'a, T: AsRef<str>>(owner: &T, repo: &T) -> Result<Vec<Release>, &'a Error> {
    let uri =
        format!(
        "https://api.github.com/repos/{}/{}/releases",
        owner.as_ref(),
        repo.as_ref(),
    );
    let body = utils::http::fetch(&String::from(uri)).unwrap();

    let releases: Vec<Release> = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(error) => {
            println!("cannot fetch latest GitHub Release: {:?}", error);
            Vec::<Release>::new()
        }
    };
    Ok(releases)
}

pub fn latest_release<'a, T: AsRef<str>>(owner: &T, repo: &T) -> Result<Release, &'a Error> {
    let releases: Vec<Release> = fetch_releases(owner, repo).unwrap();
    if releases.len() <= 0 {
        return Err(&EmptyReleasesError {});
    }
    let latest = releases
        .into_iter()
        .filter_map(|r| {
            if r.draft || r.prelease || r.assets.len() <= 0 {
                return None;
            }
            Some(r)
        })
        .next()
        .unwrap();
    Ok(latest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_release_github_hub() {
        match latest_release(&"github", &"hub") {
            Ok(release) => {
                assert!(release.assets.len() > 0);
                assert_eq!(release.draft, false);
                assert!(release.name.contains("hub"));
                assert_eq!(release.prelease, false);
                assert!(release.tag_name.contains("v"));
            }
            Err(_error) => assert!(false),
        }
    }
}