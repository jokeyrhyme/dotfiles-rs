use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use dirs::cache_dir;
use regex::Regex;
use reqwest::{Response, Url};
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use toml;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResponseMetadata {
    headers: Vec<String>,
}
impl From<&Response> for ResponseMetadata {
    fn from(res: &Response) -> ResponseMetadata {
        let mut headers = Vec::<String>::new();
        for (k, v) in res.headers().iter() {
            headers.push(format!("{}: {}", k, v.to_str().unwrap_or("[non-string]")));
        }
        ResponseMetadata { headers }
    }
}
impl From<&ResponseMetadata> for String {
    fn from(rm: &ResponseMetadata) -> String {
        toml::to_string(rm).unwrap_or_default()
    }
}

pub fn load_response(url: &Url) -> io::Result<impl Read> {
    File::open(url_body_path(url))
}

pub fn store_response(mut res: Response) -> io::Result<()> {
    let fp = url_metadata_path(res.url());
    if let Some(p) = fp.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(fp, String::from(&ResponseMetadata::from(&res)))?;

    let mut file = File::create(url_body_path(res.url()))?;
    match res.copy_to(&mut file) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
    }
}

const URL_TOO_LONG: usize = 100;

fn url_filename(url: &Url) -> String {
    let mut u = url.clone();
    if u.set_password(None).is_err() {
        return String::new();
    }
    if u.set_username("").is_err() {
        return String::new();
    }

    let mut s = String::from(u.as_str());

    if s.len() > URL_TOO_LONG {
        s.truncate(URL_TOO_LONG);

        let mut hasher = Sha256::new();
        hasher.input(u.as_str());
        s = format!("{}_{:x}", s, hasher.result());
    }

    let re = Regex::new(r"\W").expect("must parse Regex for non-word-character");
    String::from(re.replace_all(&s, "_"))
}

fn url_body_path(url: &Url) -> PathBuf {
    cache_dir()
        .expect("must find user's cache directory")
        .join(env!("CARGO_PKG_NAME"))
        .join(url_filename(url))
}

fn url_metadata_path(url: &Url) -> PathBuf {
    cache_dir()
        .expect("must find user's cache directory")
        .join(env!("CARGO_PKG_NAME"))
        .join(format!("{}.toml", url_filename(url)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_filename_does_not_include_credentials() {
        let mut url = Url::parse("https://example.com/").expect("must parse");
        url.set_username("harry").expect("must set username");
        url.set_password(Some("potter")).expect("must set password");
        let got = url_filename(&url);
        assert_eq!(got, "https___example_com_");
    }

    #[test]
    fn url_filename_truncates_and_hashes_long_inputs() {
        let url = Url::parse("https://supercalifragilisticexpialidocious.example.com/supercalifragilisticexpialidocious/supercalifragilisticexpialidocious").expect("must parse");
        let got = url_filename(&url);
        assert_eq!(got, "https___supercalifragilisticexpialidocious_example_com_supercalifragilisticexpialidocious_supercalif_fd5902017a34d8026b288be08eeda03224dfc6550f0837343e80bf92dc127071");
    }

    #[test]
    fn url_metadata_path_works() {
        let url = Url::parse("https://example.com/").expect("must parse");
        url_metadata_path(&url); // just confirming that it didn't panic
    }
}
