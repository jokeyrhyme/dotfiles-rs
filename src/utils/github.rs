use std;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::Path;
use std::str;

use serde_json;
use cabot::request::Request;

use utils;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
    state: String,
    updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
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
pub enum GitHubError {
    EmptyReleasesError,
    IoError(io::Error),
    ValidReleaseNotFoundError,
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            &GitHubError::EmptyReleasesError => fmt::Display::fmt(&"EmptyReleasesError", f),
            &GitHubError::IoError(ref err) => fmt::Display::fmt(err, f),
            &GitHubError::ValidReleaseNotFoundError => {
                fmt::Display::fmt(&"ValidReleaseNotFoundError", f)
            }
        }
    }
}

impl Error for GitHubError {
    fn cause(&self) -> Option<&Error> {
        match self {
            &GitHubError::EmptyReleasesError => None,
            &GitHubError::IoError(ref err) => Some(err as &Error),
            &GitHubError::ValidReleaseNotFoundError => None,
        }
    }

    fn description(&self) -> &str {
        match self {
            &GitHubError::EmptyReleasesError => &"EmptyReleasesError",
            &GitHubError::IoError(ref err) => err.description(),
            &GitHubError::ValidReleaseNotFoundError => &"ValidReleaseNotFoundError",
        }
    }
}

pub fn download_release_asset(asset: Asset, bin_path: &Path) {
    let req = create_request(&asset.browser_download_url);
    match utils::http::download_request(&req, &bin_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot download: {}", error);
            return;
        }
    };
    match utils::fs::set_executable(&bin_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot chmod a+rx: {}", error);
            return;
        }
    }
}

fn create_request<'a, T: AsRef<str>>(url: &T) -> Request {
    let mut headers: Vec<String> = Vec::new();

    match std::env::var("GITHUB_TOKEN") {
        Ok(token) => {
            let auth = format!("Authorization: token {}", token);
            headers.push(auth);
        }
        Err(_error) => {}
    };

    let headers_slice: Vec<&str> = headers.iter().map(|h| &**h).collect();
    utils::http::create_request(url, &headers_slice)
}

fn fetch_releases<'a, T: AsRef<str>>(owner: &T, repo: &T) -> io::Result<Vec<Release>> {
    let uri =
        format!(
        "https://api.github.com/repos/{}/{}/releases",
        owner.as_ref(),
        repo.as_ref(),
    );
    let req = create_request(&String::from(uri));
    let res = utils::http::fetch_request(&req)?;
    let body = res.body_as_string().unwrap_or_default();

    let releases: Vec<Release> = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(error) => {
            println!("cannot fetch latest GitHub Release: {:?}", error);
            Vec::<Release>::new()
        }
    };
    Ok(releases)
}

pub fn latest_release<'a, T: AsRef<str>>(owner: &T, repo: &T) -> Result<Release, GitHubError> {
    let releases = match fetch_releases(owner, repo) {
        Ok(r) => r,
        Err(error) => {
            return Err(GitHubError::IoError(error));
        }
    };
    if releases.len() <= 0 {
        return Err(GitHubError::EmptyReleasesError {});
    }
    match releases
        .into_iter()
        .filter(|r| !r.draft && !r.prelease && r.assets.len() > 0)
        .next() {
        Some(latest) => Ok(latest),
        None => Err(GitHubError::ValidReleaseNotFoundError {}),
    }
}

pub fn release_versus_current<T: AsRef<str>>(current: &T, owner: &T, repo: &T) -> Option<Release> {
    let release = match latest_release(owner, repo) {
        Ok(r) => r,
        Err(error) => {
            println!("error: {}", error);
            return None;
        }
    };

    let installed = current
        .as_ref()
        .trim_left_matches(|c: char| !c.is_digit(10))
        .trim();
    let tag_name = release.tag_name.clone();
    let latest = tag_name.trim_left_matches(|c: char| !c.is_digit(10)).trim();

    println!("current={} latest={}", &installed, &latest);

    if installed == latest {
        None
    } else {
        Some(release)
    }
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
