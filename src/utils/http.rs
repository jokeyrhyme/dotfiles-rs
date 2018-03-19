use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use futures::{Future, Stream};
use hyper::{Client, Method, Request, Uri};
use hyper::header::UserAgent;
use hyper_rustls;
use tokio_core::reactor;

pub fn download<'a, T: AsRef<str>>(url: &T, dest: &'a Path) -> Result<(), &'a Error> {
    let mut core = reactor::Core::new().unwrap();
    let uri = Uri::from_str(url.as_ref()).unwrap();

    let client = Client::configure()
        .connector(hyper_rustls::HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    let mut file = File::create(dest).expect("error creating file for download");

    let work = client.get(uri).and_then(|res| {
        res.body().for_each(
            |chunk| file.write_all(&chunk).map_err(From::from),
        )
    });

    core.run(work).unwrap();
    return Ok(());
}

pub fn fetch<'a, T: AsRef<str>>(url: &T) -> Result<Vec<u8>, &'a Error> {
    let mut core = reactor::Core::new().unwrap();
    let uri = Uri::from_str(url.as_ref()).unwrap();

    let client = Client::configure()
        .connector(hyper_rustls::HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    let mut chunks = Vec::<u8>::new();
    {
        let mut req: Request = Request::new(Method::Get, uri);
        let user_agent = format!("rust crate {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        req.headers_mut().set(UserAgent::new(user_agent));
        let work = client.request(req).and_then(|res| {
            res.body().for_each(
                |chunk| chunks.write_all(&chunk).map_err(From::from),
            )
        });
        core.run(work).unwrap();
    }

    let body = chunks.into_iter().collect();

    return Ok(body);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_google() {
        match fetch(&"https://www.google.com") {
            Ok(bytes) => {
                let body = String::from_utf8(bytes).unwrap();
                assert!(body.contains("google"));
            }
            Err(_error) => assert!(false),
        }
    }
}