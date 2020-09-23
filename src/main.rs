#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

mod finite_field_ops;
use crate::finite_field_ops::generate_multiplicative_inverse_table;

mod sub;
mod key_expansion;
mod utilities;

fn main() {
    finite_field_ops::multiply(0x5, 0x5);
    let lookup:[u8; 256] = generate_multiplicative_inverse_table();
    let i:usize = 0;
    sub::sub_word([0x9a,0x9a,0x9a,0x9a]);
    println!("{}, {}, {}, {}, {}, {}, {}, {}", lookup[i], lookup[i + 1], lookup[i + 2], lookup[i + 3], lookup[i + 4], lookup[i + 5], lookup[i + 6], lookup[i + 7] );
}


