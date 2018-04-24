use regex::Regex;

pub fn escape(pattern : &str) -> String {
    lazy_static! {
        static ref KEY: Regex = Regex::new(r"(?P<key>[?&.])").unwrap();
    }

    let exp = String::from(KEY.replace_all(pattern, r"\$key"));

    exp
}

pub fn expression(pattern : &str) -> String {
    lazy_static! {
        static ref CHARS: Regex = Regex::new(r":(?P<key>[a-zA-Z0-9_-]+)").unwrap();
    }

    let mut exp = String::from(r"^");

    exp.push_str(&CHARS.replace_all( &escape(pattern), r"(?P<$key>[a-zA-Z0-9_-]+)"));
    exp.push_str("$");
    
    exp
}

pub fn regexp(expr : &str) -> Regex {
     Regex::new(&expr).unwrap()
}