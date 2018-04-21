# url-match

[![url-match on crates.io](https://img.shields.io/crates/v/url-match.svg)](https://crates.io/crates/url-match)

> URL match patterns library.

### Demo

```shell
$ cargo run --example matcher
$ cargo run --example matchers
$ cargo run --example uri_checker
```

### Unit Test

```shell
$ cargo test
```

### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
url-match = "0.1"
```

and this to your crate root:
```rust
extern crate url_match;

use url_match::matcher;
```


### Usage

```rust
extern crate url_match;

use url_match::matcher;


fn main() {

    matcher("http://example.com/settings/:type", "http://example.com/settings/profile"); /*
    {
        type : profile
    }
    */
    matcher("http://example.com/settings/:type", "http://example.com/settings/admin"); /*
    {
        type : admin
    }
    */
    matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/repos"); /*
    {
        name : u4bi,
        type : repos
    }
    */
    matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/orgs"); /*
    {
        name : u4bi,
        type : orgs
    }
    */
    matcher("http://example.com/:name?tab=:panel", "http://example.com/u4bi?tab=stars"); /*
    {
        name  : u4bi,
        panel : stars
    }
    */
    matcher("http://example.com/:name/?tab=:panel", "http://example.com/u4bi/?tab=stars"); /*
    {
        name  : u4bi,
        panel : starts
    }
    */


    matcher(
        "http://example.com/?name=:NAME&level=:LEVEL&weapon=:WEAPON",
        "http://example.com/?name=u4bi&level=17&weapon=ak-47"); /*
    {
        NAME   : 'u4bi', 
        LEVEL  : '17', 
        WEAPON : 'ak-47'
    }
    */

    matcher("https://:SUB_DOMAIN.github.com", "https://api.github.com"); /*
    {
        SUB_DOMAIN : 'api'
    }
    */


/*
    running 8 tests
    test exam2 ... ok
    test exam3 ... ok
    test exam1 ... ok
    test exam4 ... ok
    test exam5 ... ok
    test exam6 ... ok
    test exam8 ... ok
    test exam7 ... ok

    test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

*/


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

        matcher(p, uri); /*
        {
            NAME   : 'u4bi',
            LEVEL  : '17',
            WEAPON : 'ak-47'
        }
        */

    }

}
```

___

 | Library                | URL                                                  |
 |------------------------|------------------------------------------------------|
 | url-match              | https://crates.io/crates/url-match                   |

## License
[MIT](LICENSE)