pub fn multiply(num1:u8, num2:u8) -> u8{

    //After multiplying num1 and num2 together, must take the mod of the result.
    const MOD_POLY:u16 = 0x11b;

    //Cast arguments to  u16's so they can be compared to other u16's
    let  a:u16 = num1 as u16;
    let  b:u16 = num2 as u16;


    let mut bit_position:u8 = 0;
    let mut result:u16 = 0;

    while ( 1 << bit_position) < b {
        if(b >> bit_position) & 1 == 1{
            result ^= a << bit_position;
        }
        bit_position += 1;
    }


    while result > MOD_POLY {
        //left shifts is tells the placement of the largest 1 in the bitstring 
        let mut left_shifts:u8 = 0;
        //line up leftmost digit
        while (result >> (left_shifts + 1)) >= MOD_POLY{
            left_shifts += 1;
        }
            
        //subtraction is xor in a finite field
        result ^= MOD_POLY <<  left_shifts
    }

    //After mod operation, result should be able to be contained in u8
    result as u8
}



//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(0xc1, multiply(0x57, 0x83));
    }
}
