use std::{
    self,
    env::consts::{ARCH, OS},
    error::Error,
    fmt, io,
    path::Path,
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
    CompatibleAssetNotFound,
    EmptyReleases,
    IoError(String, io::Error),
    ValidReleaseNotFound,
    WrongAssetType,
}
impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            GitHubError::CompatibleAssetNotFound => {
                write!(f, "No asset compatible with {} {}", OS, ARCH)
            }
            GitHubError::EmptyReleases => write!(f, "EmptyReleases"),
            GitHubError::IoError(ref msg, ref err) => write!(f, "{} {:?}", msg, err),
            GitHubError::ValidReleaseNotFound => write!(f, "ValidReleaseNotFound"),
            GitHubError::WrongAssetType => write!(f, "WrongAssetType"),
        }
    }
}
impl Error for GitHubError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            GitHubError::CompatibleAssetNotFound => None,
            GitHubError::EmptyReleases => None,
            GitHubError::IoError(_, ref err) => Some(err as &Error),
            GitHubError::ValidReleaseNotFound => None,
            GitHubError::WrongAssetType => None,
        }
    }
}
impl From<io::Error> for GitHubError {
    fn from(cause: io::Error) -> Self {
        GitHubError::IoError(String::new(), cause)
    }
}

pub type Result<T> = std::result::Result<T, GitHubError>;

pub fn compatible_asset(release: &Release, filter: &Fn(&Asset) -> bool) -> Result<Asset> {
    match release
        .assets
        .to_vec()
        .into_iter()
        .find(|a| filter(a) && version::is_stable(a.name.as_str()))
    {
        Some(a) => Ok(a),
        None => Err(GitHubError::CompatibleAssetNotFound {}),
    }
}

pub fn download_release_asset<P>(asset: &Asset, bin_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let req = create_request(asset.browser_download_url.clone());
    let bp = bin_path.as_ref();
    utils::http::download_request(&req, &bp)?;
    utils::fs::set_executable(&bp)?;
    Ok(())
}

fn create_request<S>(url: S) -> Request
where
    S: AsRef<str>,
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
    utils::http::create_request(url, &headers_slice)
}

fn fetch_releases<S>(owner: S, repo: S) -> Result<Vec<Release>>
where
    S: AsRef<str>,
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

pub fn fetch_tags<S>(owner: S, repo: S) -> Result<Vec<Tag>>
where
    S: AsRef<str>,
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

pub fn latest_release<S>(owner: S, repo: S) -> Result<Release>
where
    S: AsRef<str>,
{
    let releases = fetch_releases(owner, repo)?;
    if releases.is_empty() {
        return Err(GitHubError::EmptyReleases {});
    }
    match releases.into_iter().find(|r| {
        !r.draft && !r.prelease && !r.assets.is_empty() && version::is_stable(r.name.as_str())
    }) {
        Some(latest) => Ok(latest),
        None => Err(GitHubError::ValidReleaseNotFound {}),
    }
}

pub fn release_versus_current<S>(current: S, owner: S, repo: S) -> Option<Release>
where
    S: AsRef<str>,
{
    let release = match latest_release(owner, repo) {
        Ok(r) => r,
        Err(error) => {
            println!("error: {}", error);
            return None;
        }
    };

    let c = current.as_ref();
    let installed = c.trim_start_matches(|c: char| !c.is_digit(10)).trim();
    let tag_name = release.tag_name.clone();
    let latest = tag_name
        .trim_start_matches(|c: char| !c.is_digit(10))
        .trim();

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
                assert!(!tags.is_empty());
                let first = tags.into_iter().next().unwrap();
                assert!(!first.id.is_empty());
                assert!(!first.url.is_empty());
            }
            Err(_error) => assert!(false),
        }
    }

    #[test]
    fn latest_release_github_hub() {
        match latest_release("github", "hub") {
            Ok(release) => {
                assert!(!release.assets.is_empty());
                assert_eq!(release.draft, false);
                assert!(release.name.contains("hub"));
                assert_eq!(release.prelease, false);
                assert!(release.tag_name.contains('v'));
            }
            Err(_error) => assert!(false),
        }
    }
}
