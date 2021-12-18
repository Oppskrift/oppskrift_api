struct Recipe {
    name: String,
}

impl Recipe {
    fn new(name: &str) -> Self {
        Recipe {
            name: name.to_owned(),
        }
    }
}
