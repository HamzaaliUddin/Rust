fn main() {
    // u8 is data type of RUST so let's understand it (:u8) 2^8 = 256 -1  total value can store in var 255  
    // :u8 :u16 :u32 :u64

        // let num:u16 = 256;
        // println!("Hello, Rust! num = {}", num);

    // In Rust variable are immutable so we can not change it without using (mut) keyword 

        // let mut num:u16 = 256;
        // println!("Hello, Rust! num = {}", num);

        // num = 22;
        // println!("Hello, Rust! num = {}", num);

    // &str - Fixed  Length String  Special
    // String - Dynamic Length Strings - Heap Allocated

        // let string_literal:&str = "Hi Rust";
        // println!("{}",string_literal); 

        // let mut string_dynamic:String = String::from("Hi Rust ");
        // println!("{}",string_dynamic);
        // string_dynamic.push_str("I am Hamza Ali");
        // println!("{}",string_dynamic);

    // Tupple
    let emp_info:(&str,u8) = ("Hamza",21);

        // let emp_name = emp_info.0;
        // let emp_age = emp_info.1;
        // println!("Employee Name ={},Employee Age={}",emp_name,emp_age)
    
    // Distracture
    let (employee_name,empoloyee_age)= emp_info;
    println!("Employee Name ={},Employee Age={}",employee_name,empoloyee_age)

}
