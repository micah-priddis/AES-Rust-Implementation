use crate::utilities::word_from_bytes;
use crate::utilities::bytes_from_word;
use crate::sub::sub_word;



//Key expansion pseudo-code take from http://www.brainkart.com/article/AES-Key-Expansion_8410/
pub fn key_expansion(key:[u8;16], mut w:[u32; 44]) -> [u32; 44]{

    const RCON:[u32; 10] = [0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 
                            0x20000000, 0x40000000, 0x80000000, 0x1b000000, 0x36000000  ];

    const NK:usize = 4;

    let mut temp:u32;

    for i in 0..4{
        w[i] = word_from_bytes([key[4*i], key[4*i+1], key[4*i+2], key[4*i+3]]);
    }

    for i in 4..44{
        temp = w[i-1];
        if i % 4 == 0{
            temp = sub_word(rot_word(temp)) ^ RCON[ i/NK - 1 ];
        }
        w[i] = w[i-4] ^ temp;
    }

    w
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
        let mut w:[u32; 44] = [0;44];
        w = key_expansion([0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c], w);
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

    
}
