fn main(){
    let mut _v:Vec<i32> =Vec::new();
    _v.push(1);
    println!("{:?}",_v);
 
    let mut _c = Vec::<i32>::new();
    _c.push(2);
    println!("{:?}",_c);
 
    let mut _p = vec![1,2,3,4,5];
    _p.pop();
    _p.push(6);
    println!("{:?}",_p);
 
 //    read write and transfer ownersip
    let mut vector_owner: Vec<&str> = vec!["hi","Hamza"];
     write_vector(&mut vector_owner);
     println!("{:?}",vector_owner);
 
 }
 fn write_vector(vector_owner_transfer : &mut Vec<&str>){
     vector_owner_transfer.push("Rust");
     println!("{:?}",vector_owner_transfer);
 }
 
 