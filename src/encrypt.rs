use crate::sub::sub_array;
use crate::utilities::bytes_from_word;
use crate::finite_field_ops::multiply;

pub fn encrypt(in_array:[u8;16], key_schedule:[u32;44]) -> [u8;16]{



    [0;16]
}

pub fn add_round_key(mut state:[ [u8;4]; 4 ], round_key:[u32;4]) -> [ [u8;4]; 4 ]{

    for i in 0..4{
        let key:[u8;4] = bytes_from_word(round_key[i]);
        for j in 0..4{
            state[j][i] ^= key[j]; //j,i because each word is xor'ed with the column, not row
        }
    }
    state
}

fn sub_bytes(mut state:[ [u8;4]; 4 ]) -> [ [u8;4]; 4 ]{
    for i in 0..4{
        state[i] = sub_array(state[i]);
    }

    state
}

fn shift_rows(mut state:[ [u8;4]; 4 ]) -> [ [u8;4]; 4 ]{

    let mut temp:u8;

    temp = state[1][0];

    // Cyclically shift row 1 left by 1
    state[1][0] = state[1][1];
    state[1][1] = state[1][2];
    state[1][2] = state[1][3];
    state[1][3] = temp;

    
    // Cyclically shift row 2 left by 2
    // swap 1
    temp = state[2][0];
    state[2][0] = state[2][2];
    state[2][2] = temp;

    // swap 2
    temp = state[2][1];
    state[2][1] = state[2][3];
    state[2][3] = temp;


    temp = state[3][3];
    // Cyclically shift row 3 left by 3
    // same as shift right by 1
    state[3][3] = state[3][2];
    state[3][2] = state[3][1];
    state[3][1] = state[3][0];
    state[3][0] = temp;

    state
    
}

fn mix_columns(state:[ [u8;4]; 4 ]) -> [ [u8;4]; 4 ]{

    let mut altered_state:[[u8;4];4] = [[0;4];4];

    //Column assignment taken from page 18 of NIST spec
    for i in 0..4{
        altered_state[0][i] = multiply( 0x02, state[0][i]) ^ multiply(0x03, state[1][i]) ^ state[2][i] ^ state[3][i];
        altered_state[1][i] = state[0][i] ^ multiply( 0x02, state[1][i]) ^ multiply(0x03, state[2][i]) ^ state[3][i];
        altered_state[2][i] = state[0][i] ^ state[1][i] ^ multiply( 0x02, state[2][i]) ^ multiply(0x03, state[3][i]);
        altered_state[3][i] = multiply( 0x03, state[0][i]) ^ state[1][i] ^ state[2][i] ^  multiply(0x02, state[3][i]);
    }


    altered_state
}


//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mix_columns(){

        assert_eq!(
            [
                [0x8e, 0x9f, 0x01, 0xc6],
                [0x4d, 0xdc, 0x01, 0xc6],
                [0xa1, 0x58, 0x01, 0xc6],
                [0xbc, 0x9d, 0x01, 0xc6]
            ],
            mix_columns([
                [0xdb, 0xf2, 0x01, 0xc6],
                [0x13, 0x0a, 0x01, 0xc6],
                [0x53, 0x22, 0x01, 0xc6],
                [0x45, 0x5c, 0x01, 0xc6]
            ])
        );

        assert_eq!(
            [
                [0x04, 0xe0, 0x48, 0x28],
                [0x66, 0xcb, 0xf8, 0x06],
                [0x81, 0x19, 0xd3, 0x26],
                [0xe5, 0x9a, 0x7a, 0x4c]
            ],
            mix_columns([
                [0xd4, 0xe0, 0xb8, 0x1e],
                [0xbf, 0xb4, 0x41, 0x27],
                [0x5d, 0x52, 0x11, 0x98],
                [0x30, 0xae, 0xf1, 0xe5]
            ])
        );
    }

    #[test]
    fn test_shift_rows(){
        assert_eq!(
            [
                [0x00,0x01, 0x02, 0x03],
                [0x11,0x12, 0x13, 0x10],
                [0x22,0x23, 0x20, 0x21],
                [0x33,0x30, 0x31, 0x32]
            ],
            shift_rows([
                [0x00,0x01, 0x02, 0x03],
                [0x10,0x11, 0x12, 0x13],
                [0x20,0x21, 0x22, 0x23],
                [0x30,0x31, 0x32, 0x33]
            ])
        );
    }

    #[test]
    fn test_sub_bytes() {

        assert_eq!(
            [
                [0xed,0xed,0xed,0xed],
                [0xed,0xed,0xed,0xed],
                [0xed,0xed,0xed,0xed],
                [0xed,0xed,0xed,0xed],
            ],
            
            sub_bytes([
                [0x53,0x53, 0x53, 0x53],
                [0x53,0x53, 0x53, 0x53],
                [0x53,0x53, 0x53, 0x53],
                [0x53,0x53, 0x53, 0x53]
        ]));

    }

    #[test]
    fn test_add_round_key(){
        assert_eq!(
            [
                [0x19, 0xa0, 0x9a, 0xe9],
                [0x3d, 0xf4, 0xc6, 0xf8],
                [0xe3, 0xe2, 0x8d, 0x48],
                [0xbe, 0x2b, 0x2a, 0x08]
            ],
            add_round_key([
                [0x32, 0x88, 0x31, 0xe0],
                [0x43, 0x5a, 0x31, 0x37],
                [0xf6, 0x30, 0x98, 0x07],
                [0xa8, 0x8d, 0xa2, 0x34]
            ],
            [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c])
        );
    }
}