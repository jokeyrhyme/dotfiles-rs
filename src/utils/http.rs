use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;

use cabot::{request::Request, response::Response, Client, RequestBuilder};

struct HTTPCall<'a>(&'a Request, &'a Response);

impl<'a> HTTPCall<'a> {
    fn display(&self) -> String {
        let request_path = self.0.request_uri().splitn(2, '?').next().unwrap();
        format!(
            "{} {} {} {}://{}{}",
            self.1.http_version(),
            self.1.status_code(),
            self.0.http_method(),
            self.0.scheme(),
            self.0.authority(),
            request_path,
        )
    }
}

pub const EMPTY_HEADERS: &[&str] = &[];

pub fn create_request<S>(url: S, headers: &[&str]) -> Request
where
    S: Into<String>,
{
    RequestBuilder::new(url.into().as_str())
        .set_http_method("GET")
        .add_header(&format!("User-Agent: {}", user_agent()))
        .add_headers(&headers)
        .build()
        .unwrap()
}

pub fn download<P, S>(url: S, dest: P) -> io::Result<()>
where
    P: Into<PathBuf>,
    S: Into<String>,
{
    let req = create_request(url, &EMPTY_HEADERS);

    download_request(&req, dest.into())
}

pub fn download_request<P>(req: &Request, dest: P) -> io::Result<()>
where
    P: Into<PathBuf>,
{
    let res = fetch_request(&req)?;

    let d = dest.into();
    if let Some(parent) = d.parent() {
        create_dir_all(&parent)?;
    };

    let mut file = File::create(d)?;
    file.write_all(res.body().unwrap_or_default())?;

    Ok(())
}

pub fn fetch_request(req: &Request) -> io::Result<Response> {
    let client = Client::new();
    let res = match client.execute(&req) {
        Ok(r) => r,
        Err(error) => {
            return Err(io::Error::new(ErrorKind::Other, format!("{:?}", error)));
        }
    };

    match res.status_code() {
        200 => Ok(res),
        301 | 302 => {
            let headers = parse_headers(res.headers());
            let location = headers["location"].as_str();
            // TODO: send the original request's headers
            let next_request = create_request(location, &EMPTY_HEADERS);
            fetch_request(&next_request)
        }
        _ => {
            println!("{}", HTTPCall(&req, &res).display());
            println!("headers: {:?}", parse_headers(res.headers()));
            println!("{}", res.body_as_string().unwrap_or_default());
            let result = io::Error::new(ErrorKind::Other, "non-success");
            Err(result)
        }
    }
}

fn parse_headers(headers: Vec<&str>) -> HashMap<String, String> {
    // HTTP RFC2616 says duplicate headers are fine
    // but we deduplicate them here, which is fine for me for now
    let mut map = HashMap::<String, String>::new();

    for header in headers {
        let parts: Vec<&str> = header.splitn(2, ':').map(str::trim).collect();
        if parts.len() == 2 {
            let name = parts[0];
            let normalised_name = str::to_lowercase(name);
            map.insert(normalised_name, parts[1].to_string());
        }
    }
    map
}

fn user_agent() -> String {
    format!(
        "rust crate {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
}

#[cfg(test)]
mod tests {
    use cabot;

    use super::*;

    #[test]
    fn display_shows_no_search_or_fragment() {
        let req = RequestBuilder::new("https://www.google.com/?q=search#foo")
            .build()
            .unwrap();
        let res = cabot::response::ResponseBuilder::new()
            .set_status_line("HTTP/1.1 200 Ok")
            .build()
            .unwrap();
        let call = HTTPCall(&req, &res);
        assert_eq!(
            call.display(),
            "HTTP/1.1 200 GET https://www.google.com:443/"
        );
    }

    #[test]
    fn fetch_page() {
        let req = create_request("https://github.com/jokeyrhyme/dotfiles-rs", &EMPTY_HEADERS);
        let res = fetch_request(&req).unwrap();
        let body = res.body_as_string().unwrap();
        assert!(body.contains("dotfiles-rs"));
    }

    #[test]
    fn parse_sample_headers() {
        let input = vec!["", "not a key-value pair", "Name: one", "name: two"];
        let got = parse_headers(input);
        let mut want = HashMap::<String, String>::new();
        want.insert(String::from("name"), String::from("two"));
        assert_eq!(got, want);
    }
}
