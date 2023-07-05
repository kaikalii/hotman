[<img alt="github" src="https://img.shields.io/badge/GitHub-kaikalii%2Fhotman-8da0cb?logo=github">](https://github.com/kaikalii/hotman)
[<img alt="crates.io" src="https://img.shields.io/badge/crates.io-hotman-orange?logo=rust">](https://crates.io/crates/hotman)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-hotman-blue?logo=docs.rs">](https://docs.rs/hotman)

~~html~~
<br>
~~hot male~~
<br>
`hotman`

🥵 Simple HTML generation in pure Rust with no macros 🥵

See the [documentation](https://docs.rs/hotman) for usage details.

# Static Example

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

# Dynamic Example

```rust
use hotman::*;

struct User {
    id: u64,
    username: String,
    password: String,
}

impl User {
    fn new(id: u64, username: String, password: String) -> Self {
        Self {
            id,
            username,
            password,
        }
    }
}

// Some example users
let users = vec![
    User::new(0, "Alice".into(), "hunter2".into()),
    User::new(1, "Bob".into(), "swordfish".into()),
    User::new(2, "Charlie".into(), "1337".into()),
];

let users_table = table((
    Style("border-collapse: collapse;"),
    tr((th("ID"), th("Username"), th("Password"))),
    users.iter().map(|user| {
        tr((
            td(user.id.to_string()),
            td(&user.username),
            td(&user.password),
        ))
    }),
));

println!("{users_table}");
```