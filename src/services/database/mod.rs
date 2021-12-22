mod cookbook_model;
mod recipe_model;
mod user_model;

use crate::services::env::Env;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    // Database URI format : postgres://[user]:[password]@[internal_name or URL]/[database_name]
    let database_uri = Env::DatabaseUri.get_value();
    let connection_manager = ConnectionManager::<PgConnection>::new(&database_uri);

    Pool::builder()
        .build(connection_manager)
        .expect("Could not build connection pool")
}
