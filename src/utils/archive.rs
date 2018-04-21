use std;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use libflate;
use mktemp;
use tar;

use utils;

pub fn extract_gz(source: &Path, target: &Path) {
    println!("extract_gz: {} -> {}", source.display(), target.display());

    let source_file = File::open(&source).expect("unable to open source file");
    let mut source_reader = BufReader::new(source_file);

    let target_file = File::create(&target).expect("unable to open target file");
    let mut target_writer = BufWriter::new(target_file);

    let mut decoder = libflate::gzip::Decoder::new(&mut source_reader).unwrap();
    std::io::copy(&mut decoder, &mut target_writer).unwrap();
}

pub fn extract_tar(source: &Path, target: &Path, prefix: &str) {
    println!("extract_tar: {} -> {}", source.display(), target.display());

    let file = File::open(&source).expect("unable to open gzipped file");
    let mut a = tar::Archive::new(file);

    let interim_path = target.parent().unwrap().join(&prefix);

    if prefix.len() > 0 {
        utils::fs::delete_if_exists(&interim_path);
    }

    for entry in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut entry = entry.unwrap();

        let entry_path = entry.header().path().unwrap().into_owned();
        println!("{}", &entry_path.display());

        if !entry.header().entry_type().is_file() {
            continue;
        }

        if !entry_path.starts_with(&prefix) {
            continue;
        }

        let output_path: &Path;
        if prefix.len() <= 0 {
            output_path = &entry_path;
        } else {
            output_path = entry_path.strip_prefix(&prefix).unwrap();
        }

        entry.set_preserve_permissions(true);
        entry.unpack_in(&target.parent().unwrap()).unwrap();
    }

    if prefix.len() > 0 {
        std::fs::rename(&interim_path, &target).unwrap();
    }
}

pub fn extract_tar_gz(source: &Path, target: &Path, prefix: &str) {
    println!(
        "extract_tar_gz: {} -> {}",
        source.display(),
        target.display()
    );

    let temp_path;
    {
        let mut temp = mktemp::Temp::new_file().unwrap();
        temp_path = temp.to_path_buf();
        temp.release();
    }
    extract_gz(&source, &temp_path);
    extract_tar(&temp_path, &target, &prefix);

    utils::fs::delete_if_exists(&temp_path);
}
