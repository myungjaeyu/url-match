extern crate url_match;
use url_match::matcher;

fn main() {
    
    let pattern = "https://example.com/user/:user_name";
    let url     = "https://example.com/user/u4bi";


    let map = matcher(pattern, url)
                     .unwrap();

    println!("example user name");
    println!("1. {} ",   map["user_name"]);
    println!("2. {:?} ", map.get("user_name"));
    println!("3. {:?} ", map);


    /*
        example user name
        1. u4bi 
        2. Some("u4bi") 
        3. {"user_name": "u4bi"} 

    */
}