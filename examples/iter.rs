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

fn main() {
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

    println!("{users_table}")
}
