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
use key_expansion::KeyLength;
use rand::Rng;



use crate::utilities::xor_blocks;
//let mut rng = rand::thread_rng();

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

fn encrypt_file_cbc(mut input:File, mut output:File,  key_schedule:Vec<u32>, length:KeyLength) -> io::Result<()>{

    let mut initialization_vector:[u8;16] = rand::thread_rng().gen::<[u8; 16]>();

    //First 16 bytes
    output.write(&initialization_vector);

    let mut buffer:[u8;16] = [0;16];
    let mut n = input.read(&mut buffer)?;
    while n != 0 {
        buffer = xor_blocks(buffer, initialization_vector);
        initialization_vector = buffer;
        buffer = encrypt::encrypt_block(buffer, &key_schedule, length);
        output.write(&buffer);
        buffer = [0;16];
        n = input.read(&mut buffer)?;
    }
    Ok(())
}

fn decrypt_file_cbc(mut input:File, mut output:File, key_schedule:Vec<u32>, length:KeyLength) -> io::Result<()>{

    
    let mut initialization_vector:[u8;16] = [0;16];
    let mut ciphertext_block:[u8;16] = [0;16];
    let iv_size = input.read(&mut initialization_vector)?;
    if iv_size != 16{
        panic!("Error reading initialization vector!")
    }

    let mut buffer:[u8;16] = [0;16];
    let mut n = input.read(&mut buffer)?;
    while n != 0 {
        ciphertext_block = buffer;
        buffer = decrypt::decrypt_block(buffer, &key_schedule, length);
        buffer = xor_blocks(buffer, initialization_vector );
        initialization_vector = ciphertext_block;
        output.write(&buffer);
        buffer = [0;16];
        n = input.read(&mut buffer)?;
    }
    Ok(())
}

fn encrypt_file(mut input:File, mut output:File,  key_schedule:Vec<u32>, length:KeyLength) -> io::Result<()> {
    
    let mut buffer:[u8;16] = [0;16];
    let mut n = input.read(&mut buffer)?;
    while n != 0 {
        buffer = encrypt::encrypt_block(buffer, &key_schedule, length);
        output.write(&buffer);
        buffer = [0;16];
        n = input.read(&mut buffer)?;
    }
    Ok(())
}

fn decrypt_file(mut input:File, mut output:File, key_schedule:Vec<u32>, length:KeyLength) -> io::Result<()>{
    let mut buffer:[u8;16] = [0;16];
    let mut n = input.read(&mut buffer)?;
    while n != 0 {
        buffer = decrypt::decrypt_block(buffer, &key_schedule, length);
        output.write(&buffer);
        buffer = [0;16];
        n = input.read(&mut buffer)?;
    }
    Ok(())
}



fn main() -> io::Result<()> {

    let args = Opt::from_args();
    println!("Start of main");
    println!("Input: {}", args.input.display());
    println!("Output: {}", args.output.display());
    println!("Key: {}", args.key);
    println!("Decryption? {}", args.decrypt);



    let input = File::open(args.input)?;
    let output = File::create(args.output)?;

    let key:Vec<u8> = utilities::decode_key(&args.key);
    let key_schedule = key_expansion::key_expansion(&key, KeyLength::AES128);

    if args.decrypt {
        return decrypt_file_cbc(input, output, key_schedule, KeyLength::AES128);
    }
    else{
        return encrypt_file_cbc(input, output, key_schedule, KeyLength::AES128);
    }
}


