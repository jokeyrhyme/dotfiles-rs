use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Error as IOError, ErrorKind, Write};
use std::path::Path;

use cabot::{RequestBuilder, Client};
use cabot::request::Request;

pub fn create_request<'a, T: AsRef<str>>(url: &T, headers: &[&str]) -> Request {
    RequestBuilder::new(url.as_ref())
        .set_http_method("GET")
        .add_header(&format!("User-Agent: {}", user_agent()))
        .add_headers(&headers)
        .build()
        .unwrap()
}

pub fn download<'a, T: AsRef<str>>(url: &T, dest: &'a Path) -> Result<(), &'a Error> {
    let empty_headers: &[&str] = &[];
    let req = create_request(url, &empty_headers);

    download_request(req, dest)
}

pub fn download_request<'a>(req: Request, dest: &'a Path) -> Result<(), &'a Error> {
    let client = Client::new();
    let res = client.execute(&req).unwrap();

    println!("HTTP {} {}", res.status_code(), req.to_string());

    match res.status_code() {
        301 | 302 => {
            let headers = parse_headers(res.headers());
            let location = headers.get("location").unwrap().as_str();
            return download(&location, dest);
        }
        _ => {}
    };

    let mut file = File::create(dest).expect("error creating file for download");
    file.write_all(res.body().unwrap()).unwrap();

    Ok(())
}

pub fn fetch<'a, T: AsRef<str>>(url: &T) -> Result<String, IOError> {
    let empty_headers: &[&str] = &[];
    let req = create_request(url, &empty_headers);

    fetch_request(req)
}

pub fn fetch_request<'a>(req: Request) -> Result<String, IOError> {
    let client = Client::new();
    let res = client.execute(&req).unwrap();

    println!("HTTP {} {}", res.status_code(), req.to_string());

    match res.status_code() {
        200 => {}
        _ => {
            println!("headers: {:?}", parse_headers(res.headers()));
            println!("{}", res.body_as_string().unwrap_or_default());
            let result = IOError::new(ErrorKind::Other, "non-success");
            return Err(result);
        }
    };

    match res.body_as_string() {
        Ok(body) => Ok(body),
        Err(error) => {
            println!("fetch error: {:?}", error);
            println!("headers: {:?}", parse_headers(res.headers()));
            let result = IOError::new(ErrorKind::Other, format!("{:?}", error));
            Err(result)
        }
    }
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
        match fetch(&"https://github.com/jokeyrhyme/dotfiles-rs") {
            Ok(body) => {
                assert!(body.contains("dotfiles-rs"));
            }
            Err(_error) => assert!(false),
        }
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
