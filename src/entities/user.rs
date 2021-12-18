struct User {
    email: String,
}

impl User {
    fn new(email: &str) -> Self {
        User {
            email: email.to_owned(),
        }
    }
}
