use hotman::*;

fn main() {
    // A simple login page
    let dom = html((
        head((
            meta(charset("utf-8")),
            // `title` is the name of an attribute, so we use `title_elem` for the element
            title_elem("Login"),
        )),
        body((
            h1("Login"),
            form((
                (action("/login"), method("POST")),
                input((
                    type_("text"),
                    name("username"),
                    placeholder("Username"),
                    autofocus,
                )),
                input((type_("password"), name("password"), placeholder("Password"))),
                input((type_("submit"), value("Login"))),
            )),
            p((
                "Don't have an account? ",
                a((href("/register"), "Register")),
            )),
        )),
    ));
    println!("{dom}");
}
