extern crate regex;
use std::collections::HashMap;

mod util;

pub fn matcher<'a>(pattern: &'a str, url: &'a str) -> Option<HashMap<String, &'a str>> {

    let mut map = HashMap::new();

    let expr = util::expression(pattern);
    let re = util::regexp(&expr);

    if re.is_match(url) {

        let caps = re.captures(url).unwrap();

        for (index, key) in re.capture_names().enumerate() {

            if key.is_some() {
                let k = String::from(key.unwrap());
                let v = caps.get(index).unwrap().as_str();

                map.insert(k, v);

            }

        }
        
        Some(map)
    } else {
        None
    }

}