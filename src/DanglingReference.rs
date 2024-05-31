fn main(){
    //    Dangling Refernce
        let _referce_to_nothing:&String = create_string_ref();
    }
    
    fn create_string_ref()->&String {
        let s:String = String::from("Dangling Reference ");
        return &s;
    }
    
    