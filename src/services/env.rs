const HOST: &str = "HOST";
const PORT: &str = "PORT";
const DATABASE_URI: &str = "DATABASE_URI";

pub enum Env {
    Host,
    Port,
    DatabaseUri,
}

impl Env {
    pub fn get_value(&self) -> String {
        match self {
            Env::Host => dotenv::var(self::HOST).unwrap_or_else(|_| String::from("127.0.0.1")),
            Env::Port => dotenv::var(self::PORT).unwrap_or_else(|_| String::from("8080")),
            Env::DatabaseUri => dotenv::var(self::DATABASE_URI).expect("Database uri not provided"),
        }
    }
}
