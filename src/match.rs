// match like switch

fn main(){

    fn is_even_or(item:u8)->bool{
        return item % 2 == 0;
    }

    let num:u8 = 2;
    match num {
        // 1=>println!("num is one"),
        // 2 | 3 =>println!("num is one"),// 2 or 3 true
        // 4=>println!("num is one"),
                        // OR 
        num if is_even_or(num)=>println!("Correct!"),
        num if !is_even_or(num)=>println!("wrong!"),
        _=>println!("number is not reconginzible")
    }
}