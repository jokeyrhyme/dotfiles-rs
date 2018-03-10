use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use futures::{Future, Stream};
use hyper::{Client, Uri};
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
