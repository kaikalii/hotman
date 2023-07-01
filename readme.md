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
    head((meta(charset("utf-8")), title("Login"))),
    body((
        h1("Login"),
        form((
            // Attributes can be grouped, but it's not required
            (action("/login"), method("POST")),
            input((
                r#type("text"),
                name("username"),
                placeholder("Username"),
                autofocus,
            )),
            input((
                r#type("password"),
                name("password"),
                placeholder("Password"),
            )),
            input((r#type("submit"), value("Login"))),
            p((
                "Don't have an account? ",
                a((href("/register"), "Register")),
            )),
        )),
    )),
));

println!("{dom}");
```