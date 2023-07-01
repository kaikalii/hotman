use hotman::*;

fn main() {
    // A simple login page
    let dom = html((
        head((meta(charset("utf-8")), title("Login"))),
        body((
            h1("Login"),
            form((
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
            )),
        )),
    ));
    println!("{dom}");
}
