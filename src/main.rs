fn main(){
    // stack  
        let x: u8 = 21;
        let y: u8 = x;
        println!("{}",x);
        println!("{}",y);
    // heap 
    // each var have ownership so we can tranfer only so previous one got free storage  
        let first_name: String =String::from("HI"); 
        let last_name: String =  first_name;
        // println!("{}",first_name); can not call 
        println!("{}",last_name);

    
    let owner_ship:String = String::from("Hi Rust");
    ownership_func(owner_ship);
    //println!("{}",owner_ship);
}

fn ownership_func(item:String){
    println!("{}",item);
}