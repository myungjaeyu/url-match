#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::collections::HashMap;

mod util;

pub fn matcher<'a>(pattern: &'a str, url: &'a str) -> Option<HashMap<String, &'a str>> {

    let mut map = HashMap::new();

    let expr = util::expression(pattern);
    let re = util::regexp(&expr);

    let caps = re.captures(url)?;

    for (index, key) in re.capture_names().enumerate() {
        if let (Some(k), Some(c)) = (key, caps.get(index)) {
            map.insert(k.to_owned(), c.as_str());
        }
    }

    Some(map)

}
