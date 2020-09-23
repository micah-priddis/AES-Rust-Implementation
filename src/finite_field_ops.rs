pub fn multiply(num1:u8, num2:u8) -> u8{
    //The reducing polynomial used in the finite field for AES
    const REDUCING_POLY:u16 = 0x11b;
    let  a:u16;
    let  b:u16;

    //we want a to be the smaller number
    if num1 < num2{
        a = num1 as u16;
        b = num2 as u16;
    }
    else{
        b = num1 as u16;
        a = num2 as u16;
    }

    //Cast arguments to  u16's so they can be compared to other u16's
    let mut bit_position:u8 = 0;
    let mut result:u16 = 0;

    //This is essentially the multiplication algorithm with xor as the addition
    while ( 1 << bit_position) <= b {
        if(b >> bit_position) & 1 == 1{ 
            result ^= a << bit_position; //In a finite field, addition is an xor
        }
        bit_position += 1;
    }

    //255 is the greatest possible value in the finite field
    //if the result is greater than that, we take mod 0x11b of the result.
    //This is essentially the division algorithm using xor as subtraction, with the goal of finding the remainder
    while result > 255 {
        //left shifts is tells the placement of the largest 1 in the bitstring 
        let mut left_shifts:u8 = 0;
        //line up leftmost digit
        while (result >> (left_shifts + 1)) >= REDUCING_POLY{
            left_shifts += 1;
        }
            
        //subtraction is xor in a finite field
        result ^= REDUCING_POLY <<  left_shifts
    }

    //After mod operation, result should be able to be contained in u8
    result as u8
}

#[allow(dead_code)]
pub fn find_multiplicative_inverse(num:u8) -> u8{
    generate_multiplicative_inverse_table();
    num
}

pub fn generate_multiplicative_inverse_table() -> [u8; 256]{
    let mut inverse_lookup: [u8; 256] = [0; 256];
    for i in 0..256{
        for j in 0..256{
            if multiply(i as u8,j as u8) == 1 {
                inverse_lookup[i] = j as u8;
                println!("{}", j);
                break;
            }
        }
    }
    inverse_lookup

}

//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiply() {
        //Test cases derived from examples in NIST Spec and from http://www.ee.unb.ca/cgi-bin/tervo/calc2.pl?num=19&den=20&f=m&p=36&d=1
        assert_eq!(0xc1, multiply(0x57, 0x83));
        assert_eq!(0xc1, multiply(0x83, 0x57));
        assert_eq!(0xae, multiply(0x57, 0x02));
        assert_eq!(0x47, multiply(0x57, 0x04));
        assert_eq!(0x8e, multiply(0x57, 0x08));
        assert_eq!(0x07, multiply(0x57, 0x10));
        assert_eq!(92, multiply(155, 25));
        assert_eq!(92, multiply(25, 155));
        assert_eq!(103, multiply(19, 20));
        assert_eq!(103, multiply(20, 19));
        assert_eq!(11, multiply(20, 20));
        assert_eq!(19, multiply(0xFF, 0xFF));
    }
}
