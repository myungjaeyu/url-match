# url-match
> URL match patterns library.

### Demo

```shell
$ cargo run --example matcher_example
$ cargo run --example matchers_example
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

    matcher("http://example.com/settings/:type", "http://example.com/settings/profile");
    /* { 
            type : profile
        }
    */
    matcher("http://example.com/settings/:type", "http://example.com/settings/admin");
    /* {
            type : admin
        }
    */
    matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/repos");
    /* { 
            type : repos
        }
    */
    matcher("http://example.com/users/:name/:type", "http://example.com/users/u4bi/orgs");
    /* {
            type : orgs
        }
    */
    matcher("http://example.com/:name?tab=:panel", "http://example.com/u4bi?tab=stars");
    /* { 
            name  : u4bi,
            panel : stars
        }
    */
    matcher("http://example.com/:name/?tab=:panel", "http://example.com/u4bi/?tab=stars");
    /* { 
            name  : u4bi,
            panel : starts
        }
    */


    matcher(
        "http://example.com/?name=:NAME&level=:LEVEL&weapon=:WEAPON",
        "http://example.com/?name=u4bi&level=17&weapon=ak-47");
    /* { 
            NAME   : 'u4bi', 
            LEVEL  : '17', 
            WEAPON : 'ak-47'
        }
    */

    matcher("https://:SUB_DOMAIN.github.com", "https://api.github.com");
    /* { 
            SUB_DOMAIN : 'api'
        }
    */

}
```

___

 | Library                | URL                                                  |
 |------------------------|------------------------------------------------------|
 | url-match              | https://crates.io/crates/url-match                   |

## License
[MIT](LICENSE)