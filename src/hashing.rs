use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

async fn sha256_digest<R: Read>(mut reader: R) -> std::io::Result<Digest> {
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

pub async fn hash_file(path: &Path) -> std::io::Result<String> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader).await?;

    let hash_digest = HEXUPPER.encode(digest.as_ref());
    Ok(hash_digest)
}
