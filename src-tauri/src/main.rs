#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use diesel_migrations::{embed_migrations, EmbedMigrations};

// use reqwest::Url;
use tauri::App;
// use schema::todos;
use std::error::Error;
use std::{error, string, sync::Mutex};

#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate diesel_migrations;
embed_migrations!("./migrations/");

use diesel::prelude::*;
pub mod schema;
pub mod db;


fn main() {
  
  let conn = db::establish_connection();
  let state = AppState {
      count: Default::default(),
      conn: Mutex::new(db::establish_connection()),
  };
  
  diesel_migrations::run_pending_migrations(&conn).expect("Error migrating");

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![greet, unicode_data_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn unicode_data_list(state: tauri::State<AppState>) -> String{
    let con = state.conn.lock().unwrap();
    db::unicode_data_list(&con)
}

struct AppState {
  count: Mutex<i64>,
  conn: Mutex<SqliteConnection>,
}
