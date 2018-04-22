use std;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;

use libflate;
use mktemp;
use tar;
use zip;

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

pub fn extract_tar(source: &Path, target: &Path) {
    println!("extract_tar: {} -> {}", source.display(), target.display());

    let file = File::open(&source).expect("unable to open tarred file");
    let mut a = tar::Archive::new(file);

    for entry in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut entry = entry.unwrap();

        let entry_path = entry.header().path().unwrap().into_owned();

        if !entry.header().entry_type().is_file() {
            continue;
        }

        println!("{}", &entry_path.display());

        entry.set_preserve_permissions(true);
        entry.unpack_in(&target).unwrap();
    }
}

pub fn extract_tar_gz(source: &Path, target: &Path) {
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
    extract_tar(&temp_path, &target);

    utils::fs::delete_if_exists(&temp_path);
}

pub fn extract_zip(source: &Path, target: &Path) {
    println!("extract_zip: {} -> {}", source.display(), target.display());

    let zip_file = File::open(&source).expect("unable to open zipped file");
    let mut zip = zip::ZipArchive::new(zip_file).unwrap();

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).unwrap();
        let entry_path = entry.sanitized_name();

        if entry.name().ends_with("/") {
            continue; // skip directories
        }

        println!("{}", entry_path.display());

        let output_path = target.join(entry_path);
        std::fs::create_dir_all(&output_path.parent().unwrap()).unwrap();

        println!("{}", output_path.display());

        let mut output_file = File::create(&output_path).expect("unable to open destination file");
        std::io::copy(&mut entry, &mut output_file).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_fixture_foo_txt_gz() {
        let temp_path;
        {
            let mut temp = mktemp::Temp::new_file().unwrap();
            temp_path = temp.to_path_buf();
            temp.release();
        }

        let foo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foo.txt.gz");

        extract_gz(&foo_path, &temp_path);

        let extracted = File::open(&temp_path).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        assert_eq!(got.trim(), "hello, world!");

        utils::fs::delete_if_exists(&temp_path);
    }

    #[test]
    fn extract_fixture_foo_txt_tar() {
        let temp_path;
        {
            let mut temp = mktemp::Temp::new_dir().unwrap();
            temp_path = temp.to_path_buf();
            temp.release();
        }

        let foo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foo.txt.tar");

        extract_tar(&foo_path, &temp_path);

        let extracted = File::open(&temp_path.join("foo.txt")).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        assert_eq!(got.trim(), "hello, world!");

        utils::fs::delete_if_exists(&temp_path);
    }

    #[test]
    fn extract_fixture_foo_txt_zip() {
        let temp_path;
        {
            let mut temp = mktemp::Temp::new_dir().unwrap();
            temp_path = temp.to_path_buf();
            temp.release();
        }

        let foo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foo.txt.zip");

        extract_zip(&foo_path, &temp_path);

        let extracted = File::open(&temp_path.join("foo.txt")).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        assert_eq!(got.trim(), "hello, world!");

        utils::fs::delete_if_exists(&temp_path);
    }
}
