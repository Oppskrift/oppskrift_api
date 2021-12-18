const HOST: &str = "HOST";
const PORT: &str = "PORT";
const LOG_LEVEL: &str = "RUST_LOG";
const DATABASE_URI: &str = "DATABASE_URI";

pub enum Env {
    Host,
    Port,
    LogLevel,
    DatabaseUri,
}

impl Env {
    pub fn get_var(key: &Env) -> String {
        match key {
            Env::Host => dotenv::var(HOST).unwrap_or_else(|_| String::from("127.0.0.1")),
            Env::Port => dotenv::var(PORT).unwrap_or_else(|_| String::from("8080")),
            Env::LogLevel => dotenv::var(LOG_LEVEL).unwrap_or_else(|_| String::from("debug")),
            Env::DatabaseUri => dotenv::var(DATABASE_URI).expect("Database uri not provided"),
        }
    }
}
