extern crate url_match;

use url_match::matcher;

fn main() {
    
    let pattern = "https://example.com/user/:user_name";
    let url     = "https://example.com/user/u4bi";


    match matcher(pattern, url) {
        Ok(v) => println!("example user name - {}", &v["user_name"]),
        Err(v) => println!("not match {}", v)
    }


    if let Ok(ref v) = matcher(pattern, url) {
        println!("example user name - {}", &v["user_name"]);
    }


}