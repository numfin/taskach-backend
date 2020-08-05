pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn connect() -> PgConnection {
    dotenv::dotenv().ok();

    std::env::var("DATABASE_URL")
        .map(|url| PgConnection::establish(&url).expect("Error connecting to the database"))
        .expect("Error reading env::DATABASE_URL")
}
