use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use log::{error, info};
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

diesel_migrations::embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let username = env::var("DATABASE_USER").expect("Database username is not set");
        let password = env::var("DATABASE_PASSWORD").expect("Database password is not set");
        let name = env::var("DATABASE_NAME").expect("Database name is not set");
        let url = format!("postgres://{}:{}@localhost:5433/{}", username, password, name);
        let manager = ConnectionManager::<PgConnection>::new(url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    match embedded_migrations::run(&conn) {
        Ok(_) => info!("Database initialized"),
        Err(err) => error!("Failed to initialize database; {}", err),
    };
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}