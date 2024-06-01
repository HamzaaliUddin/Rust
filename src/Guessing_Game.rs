use std::io;
use rand::prelude::*;

fn main(){
    let guess_list:[&str;3] = ["apple","banana","orange"];
    let mut rng: ThreadRng = thread_rng();

    let index:usize =rng.gen_range(0..guess_list.len());
    let random_fruit:&str  = guess_list[index];
    println!("{}",random_fruit);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_)=>{
                let fruit_selected:String  =input.trim().to_lowercase();
                println!("{}",fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()){
                    println!("Fruite entered does not found !");
                    continue;
                }

                if guess_checker(&fruit_selected ,random_fruit){
                    println!("Your are winner");
                    break;
                }else{
                    println!("Retry");
                }
            }
            Err(error)=>{
                println!("Error:{}",error);
        }
        }
    }
    
}
fn guess_checker(fruit_selected:&str ,random_selected:&str)->bool{
    return   fruit_selected == random_selected;
}