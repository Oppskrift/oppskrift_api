struct Cookbook {
    name: String,
}

impl Cookbook {
    fn new(name: &str) -> Self {
        Cookbook {
            name: name.to_owned(),
        }
    }
}
