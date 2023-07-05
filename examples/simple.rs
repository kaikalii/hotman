use hotman::*;

fn main() {
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
                    On("change", "validate_username()"),
                    Autofocus,
                )),
                input((
                    Type("password"),
                    Name("password"),
                    Placeholder("Password"),
                    On("change", "validate_password()"),
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
}
