use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::{Error as IOError, ErrorKind, Write};
use std::path::Path;

use cabot::{Client, RequestBuilder, request::Request, response::Response};

pub const EMPTY_HEADERS: &[&str] = &[];

pub fn create_request<'a, T: AsRef<str>>(url: &T, headers: &[&str]) -> Request {
    RequestBuilder::new(url.as_ref())
        .set_http_method("GET")
        .add_header(&format!("User-Agent: {}", user_agent()))
        .add_headers(&headers)
        .build()
        .unwrap()
}

pub fn download<'a, T: AsRef<str>>(url: &T, dest: &'a Path) -> Result<(), &'a Error> {
    let req = create_request(url, &EMPTY_HEADERS);

    download_request(&req, dest)
}

pub fn download_request<'a>(req: &Request, dest: &'a Path) -> Result<(), &'a Error> {
    let res = fetch_request(&req).unwrap();

    match dest.parent() {
        Some(parent) => {
            create_dir_all(&parent).expect(&format!(
                "unable to create directories {}",
                &parent.display()
            ).as_str());
        }
        None => { /* probably at root directory, nothing to do */ }
    };

    let mut file = File::create(dest).expect("error creating file for download");
    file.write_all(res.body().unwrap()).unwrap();

    Ok(())
}

pub fn fetch_request<'a>(req: &Request) -> Result<Response, IOError> {
    let client = Client::new();
    let res = client.execute(&req).unwrap();

    display(&req, &res);

    match res.status_code() {
        200 => Ok(res),
        301 | 302 => {
            let headers = parse_headers(res.headers());
            let location = headers.get("location").unwrap().as_str();
            // TODO: send the origin request's headers
            let next_request = create_request(&location, &EMPTY_HEADERS);
            fetch_request(&next_request)
        }
        _ => {
            println!("headers: {:?}", parse_headers(res.headers()));
            println!("{}", res.body_as_string().unwrap_or_default());
            let result = IOError::new(ErrorKind::Other, "non-success");
            Err(result)
        }
    }
}

fn display(req: &Request, res: &Response) {
    println!(
        "{} {} {} {}://{}{}",
        res.http_version(),
        res.status_code(),
        req.http_method(),
        req.scheme(),
        req.authority(),
        req.request_uri(),
    )
}

fn parse_headers(headers: Vec<&str>) -> HashMap<String, String> {
    // HTTP RFC2616 says duplicate headers are fine
    // but we deduplicate them here, which is fine for me for now
    let mut map = HashMap::<String, String>::new();

    for header in &headers {
        let parts: Vec<&str> = header.splitn(2, ":").map(str::trim).collect();
        if parts.len() == 2 {
            let name = parts[0];
            let normalised_name = str::to_lowercase(name);
            map.insert(normalised_name, parts[1].to_string());
        }
    }

    return map;
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
    use super::*;

    #[test]
    fn fetch_page() {
        let req = create_request(&"https://github.com/jokeyrhyme/dotfiles-rs", &EMPTY_HEADERS);
        let res = fetch_request(&req).unwrap();
        let body = res.body_as_string().unwrap();
        assert!(body.contains("dotfiles-rs"));
    }

    #[test]
    fn parse_sample_headers() {
        let input = vec!["", "not a key-value pair", "Name: one", "name: two"];
        let got = parse_headers(input);
        let mut want = HashMap::<String, String>::new();
        want.insert("name".to_string(), "two".to_string());
        assert_eq!(got, want);
    }
}