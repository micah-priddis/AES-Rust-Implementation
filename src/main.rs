use std::num::Wrapping;

fn main() {
    test_vars(Wrapping(0xFF),Wrapping(0x1));
    println!("Hello, world!");
}

fn test_vars(a:Wrapping<u8>, b:Wrapping<u8> )  {
    println!("{} + {} = {}", a,b, a+b );
}
