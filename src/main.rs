
mod finite_field_ops;
use crate::finite_field_ops::generate_multiplicative_inverse_table;

fn main() {
    finite_field_ops::multiply(0x5, 0x5);
    let lookup:[u8; 256] = generate_multiplicative_inverse_table();
    let i:usize = 0;
    println!("{}, {}, {}, {}, {}, {}, {}, {}", lookup[i], lookup[i + 1], lookup[i + 2], lookup[i + 3], lookup[i + 4], lookup[i + 5], lookup[i + 6], lookup[i + 7] );
}


