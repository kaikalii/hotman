use hotman::*;

fn main() {
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
}
