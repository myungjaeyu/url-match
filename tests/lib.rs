extern crate url_match;
use url_match::matcher;

#[test]
fn exam1() {

    let map = matcher("http://example.com/settings/:type", "http://example.com/settings/profile");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam2() {

    let map = matcher("http://example.com/settings/:type", "http://example.com/settings/admin");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam3() {

    let map = matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/repos");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam4() {

    let map = matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/orgs");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam5() {

    let map = matcher("http://example.com/:name?tab=:panel", "http://example.com/u4bi?tab=stars");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam6() {

    let map = matcher("http://example.com/:name/?tab=:panel", "http://example.com/u4bi/?tab=stars");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam7() {

    let map = matcher(
                "http://example.com/?name=:NAME&level=:LEVEL&weapon=:WEAPON",
                "http://example.com/?name=u4bi&level=17&weapon=ak-47");

    assert_eq!(map.is_some(), true);

}

#[test]
fn exam8() {

    let map = matcher("https://:SUB_DOMAIN.github.com", "https://api.github.com");

    assert_eq!(map.is_some(), true);

}