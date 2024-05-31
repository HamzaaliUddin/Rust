fn main(){
    // stack  
        // let x: u8 = 21;
        // let y: u8 = x;
        // println!("{}",x);
        // println!("{}",y);
    // heap 
    // each var have ownership so we can tranfer only so previous one got free storage  
        // let first_name: String =String::from("HI"); 
        // let last_name: String =  first_name;
        // // println!("{}",first_name); can not call 
        // println!("{}",last_name);

    
    // let owner_ship:String = String::from("Hi Rust");
    // ownership_func(owner_ship);
    // // println!("{}",owner_ship);

    // let s1:String = get_string();
    // println!("This is owner_ship {}",s1);

    // let s2:String = String::from("Hi Hamza Rust!");
    // let _s3:String = send_ownership(s2);
    //     println!("recevid_ownership from s2:{}",_s3); // error because no ownership here after send _s3
    // println!("recevid_ownership from s2:{}",_s3);

     // ownership with tupple // it is fine but hard to code
        // let ownership:String =String::from("Hi Rust!!");
        // println!("{}",ownership);

        // let (_s1,_len) = tranfer_ownership(ownership);

        // println!("This is owner ship {} of length {}",_s1,_len);


    //  ownersip with Clone    // But it takes memory much
        // let ownership:String =String::from("Hi Rust!!");
        // println!("{}",ownership);
    
        // let receivedownership_len:usize = tranfer_ownership(ownership.clone());
    
        // println!("This is owner ship {} of length {}",ownership,receivedownership_len);

    // ownership with borrow so now we can read it also 
    // let ownership:String =String::from("Hi Rust!!");
    
    // let _len:usize = calulate_len(&ownership);
    //     println!("This is owner ship {} of length {}",ownership,_len);

     // ownership with borrow so now we can change
     let mut ownership:String =String::from("Hi Rust!!");
     append_string(&mut ownership);
     println!("the new string {} ",ownership);
}

// fn ownership_func(item:String){
//     println!("{}",item);
// }

// fn get_string()->String{
//     let new_string:String = String::from("Hi, Hamza");
//     return  new_string;
// }

// fn send_ownership(recevid_ownership:String )->String{
//     return recevid_ownership ;
// }



// fn tranfer_ownership(ownertrans:String)->usize{
//     let _sb = ownertrans.len();
//     return _sb;

// }

// fn calulate_len(s3:&String)->usize{
//     return s3.len();
// }


fn append_string(s3:&mut String){
    s3.push_str("Hamza")
}
