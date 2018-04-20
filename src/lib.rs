extern crate regex;
use regex::Captures;

mod util;

pub fn matcher<'a>(pattern: &'a str, url: &'a str) -> Result<Captures<'a>, bool> {

    let expr = util::expression(pattern);
    let re = util::regexp(&expr);

    if re.is_match(url) {
        Ok(re.captures(url).unwrap())
    } else {
        Err(false)
    }

}