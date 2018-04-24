extern crate url_match;
use url_match::matcher;

#[test]
fn exam1() {

    let map = matcher("http://example.com/settings/:type", "http://example.com/settings/profile");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("type").map(|e| *e), Some("profile"));

}

#[test]
fn exam2() {

    let map = matcher("http://example.com/settings/:type", "http://example.com/settings/admin");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("type").map(|e| *e), Some("admin"));

}

#[test]
fn exam3() {

    let map = matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/repos");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("name").map(|e| *e), Some("u4bi"));
    assert_eq!(map.get("type").map(|e| *e), Some("repos"));

}

#[test]
fn exam4() {

    let map = matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/orgs");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("name").map(|e| *e), Some("u4bi"));
    assert_eq!(map.get("type").map(|e| *e), Some("orgs"));

}

#[test]
fn exam5() {

    let map = matcher("http://example.com/:name?tab=:panel", "http://example.com/u4bi?tab=stars");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("name").map(|e| *e), Some("u4bi"));
    assert_eq!(map.get("panel").map(|e| *e), Some("stars"));

}

#[test]
fn exam6() {

    let map = matcher("http://example.com/:name/?tab=:panel", "http://example.com/u4bi/?tab=stars");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("name").map(|e| *e), Some("u4bi"));
    assert_eq!(map.get("panel").map(|e| *e), Some("stars"));

}

#[test]
fn exam7() {

    let map = matcher(
                "http://example.com/?name=:NAME&level=:LEVEL&weapon=:WEAPON",
                "http://example.com/?name=u4bi&level=17&weapon=ak-47");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("NAME").map(|e| *e), Some("u4bi"));
    assert_eq!(map.get("LEVEL").map(|e| *e), Some("17"));
    assert_eq!(map.get("WEAPON").map(|e| *e), Some("ak-47"));

}

#[test]
fn exam8() {

    let map = matcher("https://:SUB_DOMAIN.github.com", "https://api.github.com");

    assert!(map.is_some());
    let map = map.unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("SUB_DOMAIN").map(|e| *e), Some("api"));

}
