use regex::Regex;

pub fn escape(pattern : &str) -> String {

    let exp = String::from(Regex::new(r"(?P<key>[?&.])").unwrap().replace_all(pattern, r"\$key"));

    exp
}