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
    type CryptoOp = fn([u8;16], &Vec<u8>) -> [u8; 16];
    let operation:CryptoOp;

    let args = Opt::from_args();
    println!("Start of main");
    println!("Input: {}", args.input.display());
    println!("Output: {}", args.output.display());
    println!("Key: {}", args.key);
    println!("Decryption? {}", args.decrypt);

    if(args.decrypt){
        operation = decrypt::decrypt;
    }
    else{
        operation = encrypt::encrypt;
    }

    let mut input = File::open(args.input)?;
    let mut output = File::create(args.output)?;
    let mut buffer:[u8;16] = [0;16];
    let key:Vec<u8> = utilities::decode_key(&args.key);

    // read up to 10 bytes
    let mut n = input.read(&mut buffer)?;
    while n != 0 {
        buffer = operation(buffer, &key);
        output.write(&buffer);
        buffer = [0;16];
        n = input.read(&mut buffer)?;
    }
    
    Ok(())
}


