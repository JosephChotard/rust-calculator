#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::Mutex;

mod commands;
mod config;
mod menu;

fn main() {
  let config = config::get_config();

  tauri::Builder::default()
    .menu(menu::init())
    .manage(Mutex::new(config))
    .setup(|app| {
      let handle = app.handle();
      config::init(handle);
      Ok(())
    })
    .invoke_handler(commands::get_handlers())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
