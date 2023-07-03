use hotman::*;

struct User {
    id: u64,
    username: String,
    password: String,
}

fn main() {
    // Some example users
    let users = vec![
        User {
            id: 0,
            username: "Alice".into(),
            password: "hunter2".into(),
        },
        User {
            id: 1,
            username: "Bob".into(),
            password: "swordfish".into(),
        },
        User {
            id: 2,
            username: "Charlie".into(),
            password: "1337".into(),
        },
    ];

    let users = table((
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

    println!("{users}")
}
