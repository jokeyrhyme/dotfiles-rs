use std::fs::{create_dir_all, File};
use std::io::{self, ErrorKind};
use std::path::Path;

use reqwest::{header, Client, Request, Response, Url};

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
    match res.copy_to(&mut file) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(ErrorKind::Other, format!("{:?}", e))),
    }
}

pub fn fetch_request(req: Request) -> io::Result<Response> {
    let client = create_client();
    let res = match client.execute(req) {
        Ok(r) => r,
        Err(e) => {
            return Err(io::Error::new(ErrorKind::Other, format!("{:?}", e)));
        }
    };

    if res.status().is_success() {
        Ok(res)
    } else {
        println!("{:?} GET {}", &res.version(), &res.url());
        let result = io::Error::new(ErrorKind::Other, "non-success");
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
        let body = res.text().unwrap();
        assert!(body.contains("dotfiles-rs"));
    }
}
