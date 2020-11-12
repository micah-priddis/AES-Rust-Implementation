#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

mod finite_field_ops;

mod sub;
mod key_expansion;
mod utilities;
mod encrypt;
mod decrypt;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Opt {

    // Decryption flag. If left absent then encryption will be performed
    #[structopt(short, long)]
    decrypt: bool,

    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,

    /// The pattern to look for
    key: String,
}

fn main() -> io::Result<()> {

    let args = Opt::from_args();
    let mut input = File::open(args.input)?;
    let mut output = File::create(args.output)?;
    let mut buffer:[u8;16] = [0;16];
    let key:Vec<u8> = utilities::decode_key(&args.key);

    // read up to 10 bytes
    let mut n = input.read(&mut buffer)?;
    while n != 0 {
        buffer = encrypt::encrypt(buffer, &key);
        output.write(&buffer);
        buffer = [0;16];
        n = input.read(&mut buffer)?;
    }
    
    Ok(())
}


