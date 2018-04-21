extern crate url_match;
use url_match::matcher;

fn main() {

    let exams = vec![
        matcher("http://example.com/settings/:type",    "http://example.com/settings/profile"),
        matcher("http://example.com/settings/:type",    "http://example.com/settings/admin"),
        matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/repos"),
        matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/orgs"),
        matcher("http://example.com/:name?tab=:panel",  "http://example.com/u4bi?tab=stars"),
        matcher("http://example.com/:name/?tab=:panel", "http://example.com/u4bi/?tab=stars"),
        matcher(
            "http://example.com/?name=:NAME&level=:LEVEL&weapon=:WEAPON",
            "http://example.com/?name=u4bi&level=17&weapon=ak-47")
    ];

    for exam in &exams {

       if let &Some(ref v) = exam {

            println!("{:?}", v);

        }

    }


    /*
        {"type": "profile"}
        {"type": "admin"}
        {"name": "u4bi", "type": "repos"}
        {"name": "u4bi", "type": "orgs"}
        {"name": "u4bi", "panel": "stars"}
        {"panel": "stars", "name": "u4bi"}
        {"LEVEL": "17", "NAME": "u4bi", "WEAPON": "ak-47"}

    */

}