pub fn run() {
    println!("Hello, world!");
    println!("Running optional_enum module...");

    optional_enum();
}

fn optional_enum() {
    println!("Running optional_enum module...");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
        Active,
        Inactive,
        Pending,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct User {
        id: u32,
        name: String,
        status: Option<Status>,
    }

    let users = vec![
        User {
            id: 1,
            name: "Alice".into(),
            status: Some(Status::Active),
        },
        User {
            id: 2,
            name: "Bob".into(),
            status: None,
        },
        User {
            id: 3,
            name: "Charlie".into(),
            status: Some(Status::Pending),
        }
    ];

    for user in &users {
        if let Some(status) = &user.status {
            println!("User {} is {:?}", user.name, status);
        } else {
            println!("User {} has no status", user.name);
        }
    }
}
