use std::io;
fn main(){

    let mut input_name:String = String::new();
    println!("Enter your name !",);
    io::stdin().read_line(&mut input_name).expect("input failed!!");
    println!("Your name is {} !!",input_name);
}