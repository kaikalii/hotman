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
    head((meta(charset("utf-8")), title_elem("Login"))),
    body((
        h1("Login"),
        form((
            // Attributes can be grouped, but it's not required
            (action("/login"), method("POST")),
            input((
                type_("text"),
                name("username"),
                placeholder("Username"),
                autofocus,
            )),
            input((type_("password"), name("password"), placeholder("Password"))),
            input((type_("submit"), value("Login"))),
            p((
                "Don't have an account? ",
                a((href("/register"), "Register")),
            )),
        )),
    )),
));

println!("{dom}");
```