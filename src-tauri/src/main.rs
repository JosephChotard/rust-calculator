#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use database::get_connection;
use std::sync::Mutex;

extern crate nom;

mod commands;
mod config;
mod database;
mod maths;
mod menu;
mod parser;

fn main() {
  let config = config::get_config();

  let conn = get_connection().expect("Could not get connection");

  tauri::Builder::default()
    .menu(menu::init())
    .on_menu_event(menu::on_menu_event)
    .manage(Mutex::new(config))
    .manage(Mutex::new(conn))
    .setup(|app| {
      let handle = app.handle();
      config::init(handle);
      Ok(())
    })
    .invoke_handler(commands::get_handlers())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
