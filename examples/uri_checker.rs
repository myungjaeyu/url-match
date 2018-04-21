extern crate url_match;
use url_match::matcher;

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


    let uri = "/?name=u4bi&level=17&weapon=ak-47";


    for p in patterns {

        let map = matcher(p, uri);

        if map.is_some() {
            println!("{:?}", map.unwrap());
        }

    }


    /*
        {"LEVEL": "17", "NAME": "u4bi", "WEAPON": "ak-47"}

    */
}