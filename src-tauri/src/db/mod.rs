extern crate dotenv;

pub mod models;
use crate::schema::{unicode_data::character_name, *};
use diesel::prelude::*;
use dotenv::dotenv;
use models::UnicodeData;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    // creates a new connection to the DB and returns reference
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn unicode_data_list(conn: &SqliteConnection, top: i64, query: String) -> String {
    let all_unicode_data = unicode_data::dsl::unicode_data
        .filter(character_name.like(format!("%{}%", query)))
        .limit(top)
        .load::<UnicodeData>(conn)
        .expect("Expect loading posts");
    let serialized = serde_json::to_string(&all_unicode_data).unwrap();
    serialized
}
