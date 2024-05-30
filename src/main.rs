fn main() {
    // u8 is data type of RUST so let's understand it (:u8) 2^8 = 256 -1  total value can store in var 255  
    // :u8 :u16 :u32 :u64

        // let num:u16 = 256;
        // println!("Hello, Rust! num = {}", num);

    // In Rust variable are immutable so we can not change it without using (mut) keyword 

    let mut num:u16 = 256;
    println!("Hello, Rust! num = {}", num);

    num = 22;
    println!("Hello, Rust! num = {}", num);
}
