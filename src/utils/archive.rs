use std;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

use libflate;
use tar;
use zip;

use crate::utils::{self, fs::mkftemp};

pub fn extract_gz(source: &Path, target: &Path) -> io::Result<()> {
    let source_file = match File::open(&source) {
        Ok(f) => f,
        Err(error) => {
            println!(
                "extract_gz: unable to open source file: {}",
                source.display()
            );
            return Err(error);
        }
    };
    let mut source_reader = BufReader::new(source_file);

    let target_file = match File::create(&target) {
        Ok(f) => f,
        Err(error) => {
            println!(
                "extract_gz: unable to open target file: {}",
                target.display()
            );
            return Err(error);
        }
    };
    let mut target_writer = BufWriter::new(target_file);

    let mut decoder = libflate::gzip::Decoder::new(&mut source_reader).unwrap();
    std::io::copy(&mut decoder, &mut target_writer)?;
    Ok(())
}

pub fn extract_tar(source: &Path, target: &Path) -> io::Result<()> {
    let file = match File::open(&source) {
        Ok(f) => f,
        Err(error) => {
            println!(
                "extract_tar: unable to open source file: {}",
                source.display()
            );
            return Err(error);
        }
    };
    let mut a = tar::Archive::new(file);

    #[allow(clippy::identity_conversion)]
    for entry in a.entries()? {
        // Make sure there wasn't an I/O error
        let mut entry = entry?;

        if !entry.header().entry_type().is_file() {
            continue;
        }

        entry.set_preserve_permissions(true);
        entry.unpack_in(&target)?;
    }

    Ok(())
}

pub fn extract_tar_gz(source: &Path, target: &Path) -> io::Result<()> {
    let temp_path = mkftemp()?;
    extract_gz(&source, &temp_path)?;
    extract_tar(&temp_path, &target)?;

    utils::fs::delete_if_exists(&temp_path);
    Ok(())
}

pub fn extract_zip(source: &Path, target: &Path) -> io::Result<()> {
    extract_zip_pattern(source, target, &|_| true)
}

pub fn extract_zip_pattern(
    source: &Path,
    target: &Path,
    p: &dyn Fn(String) -> bool,
) -> io::Result<()> {
    let zip_file = match File::open(&source) {
        Ok(f) => f,
        Err(error) => {
            println!(
                "extract_zip: unable to open source file: {}",
                source.display()
            );
            return Err(error);
        }
    };
    let mut zip = zip::ZipArchive::new(zip_file)?;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let entry_path = entry.sanitized_name();

        if entry.name().ends_with('/') {
            continue; // skip directories
        }
        if !p(String::from(entry.name())) {
            continue; // skip non-matching file names
        }

        let output_path = target.join(entry_path);
        std::fs::create_dir_all(&output_path.parent().unwrap_or(&output_path))?;

        let mut output_file = File::create(&output_path)?;
        std::io::copy(&mut entry, &mut output_file)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::utils::fs::mkdtemp;

    use super::*;

    #[test]
    fn extract_fixture_foo_txt_gz() {
        let temp_path = mkftemp().unwrap();

        let foo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foo.txt.gz");

        extract_gz(&foo_path, &temp_path).expect("error");

        let extracted = File::open(&temp_path).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        assert_eq!(got.trim(), "hello, world!");

        utils::fs::delete_if_exists(&temp_path);
    }

    #[test]
    fn extract_fixture_foo_txt_tar() {
        let temp_path = mkdtemp().unwrap();

        let foo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foo.txt.tar");

        extract_tar(&foo_path, &temp_path).expect("error");

        let extracted = File::open(&temp_path.join("foo.txt")).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        assert_eq!(got.trim(), "hello, world!");

        utils::fs::delete_if_exists(&temp_path);
    }

    #[test]
    fn extract_fixture_foo_txt_zip() {
        let temp_path = mkdtemp().unwrap();

        let foo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foo.txt.zip");

        extract_zip(&foo_path, &temp_path).expect("error");

        let extracted = File::open(&temp_path.join("foo.txt")).unwrap();
        let mut reader = BufReader::new(extracted);
        let mut got = String::new();
        reader.read_to_string(&mut got).unwrap();

        assert_eq!(got.trim(), "hello, world!");

        utils::fs::delete_if_exists(&temp_path);
    }
}
