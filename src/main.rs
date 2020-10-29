#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::io::prelude::*;
use std::fs::File;
use std::env;


mod finite_field_ops;

mod sub;
mod key_expansion;
mod utilities;
mod encrypt;
mod decrypt;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    if &args[1] == "e" {
        let filename = &args[2];
        let outfile = &args[3];
        let key = &args[4];
    
        println!("File to be encrypted: {}", filename);
        println!("Encrypted file: {}", outfile);
        println!("Key used: {}", key);
    
        encrypt_file(&filename, outfile, key);
    }

}

fn parse_key(key:&String) -> [u8;16]{
    println!("key as string from cmdl {}", key);
    let num:u128 = key.parse::<u128>().unwrap();
    println!("parsed number {:x?}", num);
    let key_array = utilities::bytes_from_u128(num);

    key_array
}



fn encrypt_file(input_file:&String, output_file:&String, key:&String ) -> std::io::Result<()> {

    
    let key_schedule = key_expansion::key_expansion(parse_key(key));

    let mut input = File::open(input_file)?;
    let mut output = File::create(output_file)?;

    let file_size = input.metadata().unwrap().len();
    let num_of_blocks = file_size / 16 + 1;
    let mut num_of_bytes = 0;


    let mut bytes:[u8;16] = [0;16];

    for i in 0..num_of_blocks{
        bytes = [0;16];
        input.read(&mut bytes)?;

        bytes = encrypt::encrypt(bytes, key_schedule);

        output.write(&mut bytes)?;
        num_of_bytes += 16;
    }
    Ok(())
}



//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word_from_bytes() {
        //assert_eq!(0x85342591, word_from_bytes( [0x85, 0x34, 0x25, 0x91]) );
        
    }

}
