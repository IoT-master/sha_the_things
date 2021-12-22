use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA512};
use std::env::args;
use std::fs::File;
use std::io::Error;
use std::io::{BufReader, Read};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
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

fn sha512_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA512);
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

fn sha1_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
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

fn main() -> Result<(), Error> {
    let my_args: Vec<String> = args().collect();
    let mut i = 1;

    while i < my_args.len() {
        let path = &my_args[i];

        let input = File::open(path)?;
        let mut reader = BufReader::new(input);
        let digest_1 = sha1_digest(reader.by_ref())?;
        let digest_256 = sha256_digest(reader.by_ref())?;
        let digest_512 = sha512_digest(reader.by_ref())?;

        
        println!("   File: {}", &my_args[i]);
        println!("   {}", "=".repeat(50));
        println!("  SHA-1: {}", HEXLOWER.encode(digest_1.as_ref()));
        println!("SHA-256: {}", HEXLOWER.encode(digest_256.as_ref()));
        println!("SHA-512: {}", HEXLOWER.encode(digest_512.as_ref()));
        println!("");
        i +=1;
    }
    Ok(())
}
