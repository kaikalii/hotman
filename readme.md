~~html~~
<br>
~~hot male~~
<br>
`hotman`

This crate provides a simple way to generate HTML elements in pure Rust.

# Example

This example looks better with proper language server syntax highlighting
because tags are functions and attributes are structs.
```rust
use hotman::*;

let dom = html((
    Comment("A simple login page"),
    head((meta(Charset("utf-8")), title("Login"))),
    body((
        h1("Login"),
        form((
            (Action("/login"), Method("POST")),
            input((
                Type("text"),
                Name("username"),
                Placeholder("Username"),
                Autofocus,
            )),
            input((Type("password"), Name("password"), Placeholder("Password"))),
            input((Type("submit"), Value("Login"))),
        )),
        p((
            "Don't have an account? ",
            a((Href("/register"), "Register")),
        )),
    )),
))
.page();

println!("{dom}");
```