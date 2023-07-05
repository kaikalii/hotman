[<img alt="github" src="https://img.shields.io/badge/GitHub-kaikalii%2Fhotman-8da0cb?logo=github">](https://github.com/kaikalii/hotman)
[<img alt="crates.io" src="https://img.shields.io/badge/crates.io-hotman-orange?logo=rust">](https://crates.io/crates/hotman)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-hotman-blue?logo=docs.rs">](https://docs.rs/hotman)

~~html~~
<br>
~~hot male~~
<br>
`hotman`

This crate provides a simple way to generate HTML elements in pure Rust.

# Example

```rust
use hotman::*;

let dom = html((
    Comment("A simple login page"),
    head((
        meta(Charset("utf-8")),
        title("Login"),
        script(Src("/script.js")),
    )),
    body((
        h1("Login"),
        form((
            (Action("/login"), Method("POST")),
            input((
                Type("text"),
                Name("username"),
                Placeholder("Username"),
                On(Change, "validate_username()"),
                Autofocus,
            )),
            input((
                Type("password"),
                Name("password"),
                Placeholder("Password"),
                On(Change, "validate_password()"),
            )),
            input((Type("submit"), Value("Login"))),
        )),
        BR,
        p((
            "Don't have an account? ",
            a((Href("/register"), "Register")),
        )),
    )),
))
.page();

println!("{dom}");
```