use hotman::*;

fn main() {
    let dom = html((
        head((meta(charset("utf-8")), title("Catalog"))),
        body(div((
            class("centered"),
            div((h3("Catalog"), h1("Login"), form(()))),
        ))),
    ));
    println!("{dom}");
}
