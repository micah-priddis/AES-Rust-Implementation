use hex::decode;

pub fn word_from_bytes(bytes:[u8; 4])->u32{
    let mut word:u32 = bytes[3] as u32;
    word += (bytes[2] as u32 ) << 8;
    word += (bytes[1] as u32 ) << 16;
    word += (bytes[0] as u32 ) << 24;

    word
}

pub fn bytes_from_word(word:u32)->[u8;4]{
    let mut bytes:[u8;4] = [0;4];
    bytes[3] = (word & 0xFF ) as u8;
    bytes[2] = ((word & 0xFF00) >> 8 ) as u8;
    bytes[1] = ((word & 0xFF0000) >> 16 ) as u8;
    bytes[0] = ((word & 0xFF000000) >> 24 ) as u8;

    bytes
}

pub fn print_matrix(matrix:&[[u8;4];4]){
    for i in 0..4{
        println!("{:x?} {:x?} {:x?} {:x?} ", matrix[i][0], matrix[i][1], matrix[i][2], matrix[i][3] );
    }
    println!();
}

pub fn round_string(matrix:&[[u8;4];4]){
    for i in 0..4{
        print!("{:02x?}{:02x?}{:02x?}{:02x?}", matrix[0][i], matrix[1][i], matrix[2][i], matrix[3][i] );
    }
    println!();
}

pub fn decode_key(s: &str) -> Vec<u8> {
    if s.len() != 32 && s.len() != 48 && s.len() != 64 {
        panic!("Key must be a string of 32 characters");
    }
    decode(s).unwrap()
}


//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word_from_bytes() {
        assert_eq!(0x85342591, word_from_bytes( [0x85, 0x34, 0x25, 0x91]) );
        assert_eq!(0xaece3212, word_from_bytes( [0xae, 0xce, 0x32, 0x12]) );
        assert_eq!(0xb5000474, word_from_bytes( [0xb5, 0x00, 0x04, 0x74]) );
        assert_eq!(0x2b7e1516, word_from_bytes( [0x2b, 0x7e, 0x15, 0x16]) );
        
    }

    #[test]
    fn test_bytes_from_word() {
        assert_eq!([0x85, 0x34, 0x25, 0x91], bytes_from_word( 0x85342591 ) );
        assert_eq!([0xae, 0xce, 0x32, 0x12], bytes_from_word( 0xaece3212 ) );
        assert_eq!([0xb5, 0x00, 0x04, 0x74], bytes_from_word( 0xb5000474 ) );
    }

    #[test]
    fn test_decode_hex() {
        let expected:Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        assert_eq!(expected, decode_key("2b7e151628aed2a6abf7158809cf4f3c") );
    }
}