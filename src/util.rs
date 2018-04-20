use regex::Regex;

pub fn escape(pattern : &str) -> String {

    let exp = String::from(Regex::new(r"(?P<key>[?&.])").unwrap().replace_all(pattern, r"\$key"));

    exp
}

pub fn expression(pattern : &str) -> String {

    let chars = Regex::new(r":(?P<key>[a-zA-Z0-9_-]+)").unwrap();

    let mut exp = String::from(r"^");

    exp.push_str(&chars.replace_all( &escape(pattern), r"(?P<$key>[a-zA-Z0-9_-]+)"));
    exp.push_str("$");
    
    exp
}