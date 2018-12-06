use std::{
    self,
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
    str,
};

use cabot::request::Request;
use serde_derive::Deserialize;
use serde_json;

use crate::lib::version;
use crate::utils;

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

#[derive(Clone, Debug, Deserialize)]
pub struct Tag {
    #[serde(rename = "ref")]
    pub id: String,
    pub url: String,
}

#[derive(Debug)]
pub enum GitHubError {
    EmptyReleasesError,
    IoError(io::Error),
    ValidReleaseNotFoundError,
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            GitHubError::EmptyReleasesError => fmt::Display::fmt(&"EmptyReleasesError", f),
            GitHubError::IoError(ref err) => fmt::Display::fmt(err, f),
            GitHubError::ValidReleaseNotFoundError => {
                fmt::Display::fmt(&"ValidReleaseNotFoundError", f)
            }
        }
    }
}

impl Error for GitHubError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            GitHubError::EmptyReleasesError => None,
            GitHubError::IoError(ref err) => Some(err as &Error),
            GitHubError::ValidReleaseNotFoundError => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            GitHubError::EmptyReleasesError => &"EmptyReleasesError",
            GitHubError::IoError(ref err) => err.description(),
            GitHubError::ValidReleaseNotFoundError => &"ValidReleaseNotFoundError",
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn download_release_asset<P>(asset: &Asset, bin_path: P)
where
    P: Into<PathBuf> + AsRef<Path>,
{
    let req = create_request(asset.browser_download_url.clone());
    match utils::http::download_request(&req, bin_path.as_ref()) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot download: {}", error);
            return;
        }
    };
    match utils::fs::set_executable(bin_path.as_ref()) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot chmod a+rx: {}", error);
            return;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn create_request<S>(url: S) -> Request
where
    S: Into<String> + AsRef<str>,
{
    let mut headers: Vec<String> = Vec::new();

    match std::env::var("GITHUB_TOKEN") {
        Ok(token) => {
            let auth = format!("Authorization: token {}", token);
            headers.push(auth);
        }
        Err(_error) => {}
    };

    let headers_slice: Vec<&str> = headers.iter().map(|h| &**h).collect();
    utils::http::create_request(url.as_ref(), &headers_slice)
}

#[allow(clippy::needless_pass_by_value)]
fn fetch_releases<S>(owner: S, repo: S) -> io::Result<Vec<Release>>
where
    S: Into<String> + AsRef<str>,
{
    let uri = format!(
        "https://api.github.com/repos/{}/{}/releases",
        owner.as_ref(),
        repo.as_ref(),
    );
    let req = create_request(uri);
    let res = utils::http::fetch_request(&req)?;
    let body = res.body_as_string().unwrap_or_default();

    let releases: Vec<Release> = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(error) => {
            println!("cannot fetch GitHub Releases: {:?}", error);
            Vec::<Release>::new()
        }
    };
    Ok(releases)
}

#[allow(clippy::needless_pass_by_value)]
pub fn fetch_tags<S>(owner: S, repo: S) -> io::Result<Vec<Tag>>
where
    S: Into<String> + AsRef<str>,
{
    let uri = format!(
        "https://api.github.com/repos/{}/{}/git/refs/tags",
        owner.as_ref(),
        repo.as_ref(),
    );
    let req = create_request(uri);
    let res = utils::http::fetch_request(&req)?;
    let body = res.body_as_string().unwrap_or_default();

    let tags: Vec<Tag> = match serde_json::from_str(&body) {
        Ok(t) => t,
        Err(error) => {
            println!("cannot fetch GitHub tags: {:?}", error);
            Vec::<Tag>::new()
        }
    };
    Ok(tags
        .into_iter()
        .map(|t| Tag {
            id: str::replace(&t.id, "refs/tags/", ""),
            url: t.url,
        })
        .collect())
}

#[allow(clippy::needless_pass_by_value)]
pub fn latest_release<S>(owner: S, repo: S) -> Result<Release, GitHubError>
where
    S: Into<String> + AsRef<str>,
{
    let releases = match fetch_releases(owner, repo) {
        Ok(r) => r,
        Err(error) => {
            return Err(GitHubError::IoError(error));
        }
    };
    if releases.is_empty() {
        return Err(GitHubError::EmptyReleasesError {});
    }
    match releases.into_iter().find(|r| {
        !r.draft && !r.prelease && !r.assets.is_empty() && version::is_stable(r.name.as_str())
    }) {
        Some(latest) => Ok(latest),
        None => Err(GitHubError::ValidReleaseNotFoundError {}),
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn release_versus_current<S>(current: S, owner: S, repo: S) -> Option<Release>
where
    S: Into<String> + AsRef<str>,
{
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
    fn fetch_tags_github_hub() {
        match fetch_tags("github", "hub") {
            Ok(tags) => {
                assert!(tags.len() > 0);
                let first = tags.into_iter().next().unwrap();
                assert!(first.id.len() > 0);
                assert!(first.url.len() > 0);
            }
            Err(_error) => assert!(false),
        }
    }

    #[test]
    fn latest_release_github_hub() {
        match latest_release("github", "hub") {
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
