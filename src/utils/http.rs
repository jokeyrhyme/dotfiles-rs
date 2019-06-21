use std::fs::{create_dir_all, File};
use std::io::{self, Read};
use std::path::Path;

use chrono::{offset::Utc, Duration};
use reqwest::{header, Client, Request, Url};

use crate::lib::cache;

pub fn create_request<S>(url: S, headers: Option<header::HeaderMap>) -> Request
where
    S: AsRef<str>,
{
    create_client()
        .get(Url::parse(url.as_ref()).expect("parse URL"))
        .header("User-Agent", user_agent())
        .headers(match headers {
            Some(h) => h,
            None => header::HeaderMap::new(),
        })
        .build()
        .expect("new HTTP(S) request")
}

pub fn download<P, S>(url: S, dest: P) -> io::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let req = create_request(url, None);

    download_request(req, dest.as_ref())
}

pub fn download_request<P>(req: Request, dest: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut res = fetch_request(req)?;

    let d = dest.as_ref();
    if let Some(parent) = d.parent() {
        create_dir_all(&parent)?;
    };

    let mut file = File::create(d)?;
    match io::copy(&mut res, &mut file) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
    }
}

pub fn fetch_request(req: Request) -> io::Result<impl Read> {
    let url = req.url().clone();
    if let Ok(rm) = cache::load_response_metadata(&url) {
        let a_while_ago = Utc::now() - Duration::minutes(15);
        if rm.date > a_while_ago {
            return cache::load_response_body(&url);
        }
    }
    // proceed with fresh HTTP request

    let client = create_client();
    let res = match client.execute(req) {
        Ok(r) => r,
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e)));
        }
    };

    if res.status().is_success() {
        cache::store_response(&url, res)?;
        cache::load_response_body(&url)
    } else {
        println!("{:?} GET {}", &res.version(), &res.url());
        let result = io::Error::new(io::ErrorKind::Other, "non-success");
        Err(result)
    }
}

fn create_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, user_agent());
    Client::builder()
        .default_headers(headers)
        .build()
        .expect("new HTTP(S) client")
}

fn user_agent() -> header::HeaderValue {
    header::HeaderValue::from_str(&format!(
        "rust crate {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))
    .expect("User-Agent header")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_page() {
        let req = create_request("https://github.com/jokeyrhyme/dotfiles-rs", None);
        let mut res = fetch_request(req).unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        assert!(body.contains("dotfiles-rs"));
    }
}
