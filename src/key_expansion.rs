pub fn rot_word(mut word:[u8; 4]) -> [u8;4]{
    let temp:u8 = word[0];

    word[0] = word[1];
    word[1] = word[2];
    word[2] = word[3];
    word[3] = temp;

    word
}

//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rot_word() {
        assert_eq!([2,3,4,1] , rot_word([1,2,3,4]));
        assert_eq!([5,6,7,4] , rot_word([4,5,6,7]));
        assert_eq!([10,0,0,10], rot_word([10,10,0,0]))
    }
}
