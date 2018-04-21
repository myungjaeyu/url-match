extern crate url_match;
use url_match::matcher;


fn uri_checker(pattern: &str, url: &str) {

    if let Ok(ref v) = matcher(pattern, url) {

        for index in 1..v.len() {
            print!("{} /", &v[index]);
        }

        print!("\n");
    }

}


fn main() {

    let patterns = vec![
                    "/settings/:type",
                    "/settings/:type",
                    "/users/:name/:type",
                    "/users/:name/:type",
                    "/:name?tab=:panel",
                    "/:name/?tab=:panel",
                    "/?name=:NAME&level=:LEVEL&weapon=:WEAPON"
                ];


    let url = "/?name=u4bi&level=17&weapon=ak-47";


    for exam in patterns {

        uri_checker(exam, url);

    }

}