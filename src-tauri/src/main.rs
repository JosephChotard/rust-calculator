#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod config;
mod menu;

fn main() {
  tauri::Builder::default()
    .menu(menu::init())
    .setup(|app| {
      let handle = app.handle();
      config::init(handle);
      Ok(())
    })
    .invoke_handler(commands::get_handlers())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
