use crate::sub::inv_sub_array;
use crate::finite_field_ops::multiply;
use crate::key_expansion::key_expansion;
use crate::utilities::print_matrix;
use crate::encrypt::add_round_key;
use crate::utilities::round_string;

fn inv_sub_bytes(mut state:[ [u8;4]; 4 ]) -> [ [u8;4]; 4 ]{
    for i in 0..4{
        state[i] = inv_sub_array(state[i]);
    }

    state
}

fn inv_shift_rows(mut state:[ [u8;4]; 4 ]) -> [ [u8;4]; 4 ]{
    let mut temp:u8;
    temp = state[1][3];
    state[1][3] = state[1][2];
    state[1][2] = state[1][1];
    state[1][1] = state[1][0];
    state[1][0] = temp;

    // Cyclically shift row 2 left by 2
    // swap 1
    temp = state[2][0];
    state[2][0] = state[2][2];
    state[2][2] = temp;

    // swap 2
    temp = state[2][1];
    state[2][1] = state[2][3];
    state[2][3] = temp;

    temp = state[3][0];
    state[3][0] = state[3][1];
    state[3][1] = state[3][2];
    state[3][2] = state[3][3];
    state[3][3] = temp;


    state 
}

fn inv_mix_columns(state:[ [u8;4]; 4 ]) -> [ [u8;4]; 4 ]{

    let mut altered_state:[[u8;4];4] = [[0;4];4];

    //Column assignment taken from page 18 of NIST spec
    for i in 0..4{
        altered_state[0][i] = multiply( 0x0e, state[0][i]) ^ multiply(0x0b, state[1][i]) ^ multiply(0x0d,state[2][i]) ^ multiply(0x09, state[3][i]);
        altered_state[1][i] = multiply( 0x09, state[0][i]) ^ multiply(0x0e, state[1][i]) ^ multiply(0x0b,state[2][i]) ^ multiply(0x0d, state[3][i]);
        altered_state[2][i] = multiply( 0x0d, state[0][i]) ^ multiply(0x09, state[1][i]) ^ multiply(0x0e,state[2][i]) ^ multiply(0x0b, state[3][i]);
        altered_state[3][i] = multiply( 0x0b, state[0][i]) ^ multiply(0x0d, state[1][i]) ^ multiply(0x09,state[2][i]) ^ multiply(0x0e, state[3][i]);
    }


    altered_state
}


pub fn decrypt(in_array:[u8;16], key:&Vec<u8>) -> [u8;16]{

    let key_schedule:[u32;44] = key_expansion(&key);
    let mut state:[[u8;4];4] = [[0;4];4];

    //Map input bytes to state 
    for i in 0..16{
        state[i % 4][i / 4] = in_array[i]; //Row of bytes is given by i % 4, column is given by i / 4
    }

    print_matrix(&state);

    state = add_round_key(state,  &key_schedule[40..44]);
    

    for round in (1..10).rev(){
        //println!("Start of round {}", round);
        //round_string(&state);
        println!();
        state = inv_shift_rows(state);
        state = inv_sub_bytes(state);
        state = add_round_key(state,  &key_schedule[round*4..(round+1)*4]);
        state = inv_mix_columns(state);
    }

    state = inv_shift_rows(state);
    state = inv_sub_bytes(state);
    state = add_round_key(state,  &key_schedule[0..4]);

    round_string(&state);
    let mut result:[u8;16] = [0;16];
    for i in 0..16{
        result[i] = state[i % 4][i / 4];
    }
    result
}

//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decrypt(){
        let key:Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];
        assert_eq!([0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff], //plaintext
            decrypt(
                [0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a], //ciphertext 
                &key //key
            ));
    }

}