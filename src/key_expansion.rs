use crate::utilities::word_from_bytes;
use crate::utilities::bytes_from_word;
use crate::sub::sub_word;

#[derive(PartialEq)]
pub enum KeyLength {
    AES128,
    AES192,
    AES256,
}


//based on Key expansion pseudo-code  from http://www.brainkart.com/article/AES-Key-Expansion_8410/
pub fn key_expansion(key:&Vec<u8>, length:KeyLength) -> Vec<u32>{
    let mut w:Vec<u32>;

    let mut key_word_len:usize = 4;

    match length {
        KeyLength::AES128 => { w = vec![0;44]; key_word_len = 4; }
        KeyLength::AES192 => { w = vec![0;52]; key_word_len = 6; }
        KeyLength::AES256 => { w = vec![0;60]; key_word_len = 8; }
    }

    const RCON:[u32; 10] = [0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 
                            0x20000000, 0x40000000, 0x80000000, 0x1b000000, 0x36000000  ];


    
    for i in 0..key_word_len{
        w[i] = word_from_bytes([key[4*i], key[4*i+1], key[4*i+2], key[4*i+3]]);
    }
    let mut temp:u32;

    for i in key_word_len..w.len(){
        temp = w[i-1];
        if i % key_word_len == 0{
            temp = sub_word(rot_word(temp)) ^ RCON[ i/key_word_len - 1 ];
        }
        else if length == KeyLength::AES256 && i % 8 == 4{
            temp = sub_word(temp);
        }
        w[i] = w[i-key_word_len] ^ temp;
    }

    w //Return key schedule
}

pub fn rot_bytes(mut word:[u8; 4]) -> [u8;4]{
    let temp:u8 = word[0];

    word[0] = word[1];
    word[1] = word[2];
    word[2] = word[3];
    word[3] = temp;

    word
}

pub fn rot_word(mut word:u32) -> u32{
    let mut bytes:[u8;4] = bytes_from_word(word);
    let temp:u8 = bytes[0];

    bytes[0] = bytes[1];
    bytes[1] = bytes[2];
    bytes[2] = bytes[3];
    bytes[3] = temp;

    word = word_from_bytes(bytes);

    word
}

//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rot_bytes() {
        assert_eq!([2,3,4,1] , rot_bytes([1,2,3,4]));
        assert_eq!([5,6,7,4] , rot_bytes([4,5,6,7]));
        assert_eq!([10,0,0,10], rot_bytes([10,10,0,0]))
    }

    #[test]
    fn test_rot_word() {
        assert_eq!(0x23456701, rot_word(0x01234567));
        assert_eq!(0x02030401, rot_word(0x01020304));
        assert_eq!(0x10000010, rot_word(0x10100000));
    }

    #[test]
    fn test_128bit_key_expansion() {
        //Example derived from Appendix A in the NIST specification. Linked in readme
        let key:Vec<u8> = vec![0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c];
        let w:Vec<u32> = key_expansion(&key, KeyLength::AES128);
        assert_eq!( 0x2b7e1516, w[0] );
        assert_eq!( 0x28aed2a6, w[1] );
        assert_eq!( 0xabf71588, w[2] );
        assert_eq!( 0x09cf4f3c, w[3] );
        assert_eq!( 0xa0fafe17, w[4] );
        assert_eq!( 0x88542cb1, w[5] );
        assert_eq!( 0xd4d1c6f8, w[20] );
        assert_eq!( 0x7c839d87, w[21] );
        assert_eq!( 0xac7766f3, w[36] );
        assert_eq!( 0xd014f9a8, w[40] );
        assert_eq!( 0xb6630ca6, w[43] );
         
    }

    #[test]
    fn test_192bit_key_expansion() {
        //Example derived from Appendix B in the NIST specification. Linked in readme
        let key:Vec<u8> = vec![0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90, 0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b];
        let w:Vec<u32> = key_expansion(&key, KeyLength::AES192);
        assert_eq!( 0x8e73b0f7, w[0] );
        assert_eq!( 0xda0e6452, w[1] );
        // assert_eq!( 0xabf71588, w[2] );
        // assert_eq!( 0x09cf4f3c, w[3] );
        // assert_eq!( 0xa0fafe17, w[4] );
        // assert_eq!( 0x88542cb1, w[5] );
        assert_eq!( 0xfe0c91f7, w[6] );
        assert_eq!( 0x2402f5a5, w[7] );
        assert_eq!( 0x4db7b4bd, w[12] );
        assert_eq!( 0x4d6dce24, w[21] );
        // assert_eq!( 0xac7766f3, w[36] );
        // assert_eq!( 0xd014f9a8, w[40] );
        assert_eq!( 0x01002202, w[51] );
    }

    #[test]
    fn test_256bit_key_expansion() {
        //Example derived from Appendix B in the NIST specification. Linked in readme
        let key:Vec<u8> = vec![0x60,0x3d,0xeb,0x10,0x15,0xca,0x71,0xbe,0x2b,0x73,0xae,0xf0,0x85,0x7d,0x77,0x81, 0x1f,0x35,0x2c,0x07,0x3b,0x61,0x08,0xd7,0x2d,0x98,0x10,0xa3,0x09,0x14,0xdf,0xf4];
        let w:Vec<u32> = key_expansion(&key, KeyLength::AES256);
        assert_eq!( 0x603deb10, w[0] );
        assert_eq!( 0x857d7781, w[3] );
        // assert_eq!( 0x09cf4f3c, w[3] );
        // assert_eq!( 0xa0fafe17, w[4] );
        // assert_eq!( 0x88542cb1, w[5] );
        assert_eq!( 0x2d9810a3, w[6] );
        assert_eq!( 0x0914dff4, w[7] );
        assert_eq!( 0x9ba35411, w[8] );
        assert_eq!( 0x8e6925af, w[9] );
        assert_eq!( 0xa51a8b5f, w[10] );
        assert_eq!( 0x2067fcde, w[11] );
        assert_eq!( 0xa8b09c1a, w[12] );
        
        assert_eq!( 0x2678a647, w[21] );
        // assert_eq!( 0xac7766f3, w[36] );
        // assert_eq!( 0xd014f9a8, w[40] );
        assert_eq!( 0x706c631e, w[59] );
    }
}
