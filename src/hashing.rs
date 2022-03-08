use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use walkdir::WalkDir;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

pub fn hash_tree() -> std::io::Result<()>{
    let start_path = Path::new(".");
    for entry in WalkDir::new(start_path) {
        hash_file(entry.unwrap().path());
    }

    Ok(())
}

fn sha256_digest<R: Read>(mut reader: R) -> std::io::Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn hash_file(path: &Path) -> std::io::Result<String> {

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    let hash_digest = HEXUPPER.encode(digest.as_ref());
    println!("SHA-256 digest is {}", hash_digest);

    Ok(hash_digest)
}
