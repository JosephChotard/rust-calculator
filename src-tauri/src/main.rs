#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use database::get_connection;
use parser::builtin;
use std::sync::Mutex;

extern crate nom;

mod commands;
mod database;
mod maths;
mod menu;
mod operations;
mod parser;
mod system;

fn main() {
  let config = system::get_config();

  let conn = get_connection().expect("Could not get connection");
  let context = builtin();

  tauri::Builder::default()
    .menu(menu::init())
    .on_menu_event(menu::on_menu_event)
    .manage(Mutex::new(config))
    .manage(Mutex::new(conn))
    .manage(Mutex::new(context))
    .invoke_handler(commands::get_handlers())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
