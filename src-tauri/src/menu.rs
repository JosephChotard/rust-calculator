use super::database::clear_operation_history;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowMenuEvent};

pub fn init() -> Menu {
  let root_submenu = Submenu::new("", Menu::new().add_native_item(MenuItem::Quit));
  let clear_history = CustomMenuItem::new("clear_history", "Clear History");
  let editmenu = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste)
      .add_native_item(MenuItem::SelectAll)
      .add_item(clear_history),
  );
  let menu = Menu::new().add_submenu(root_submenu).add_submenu(editmenu);
  menu
}

pub fn on_menu_event(event: WindowMenuEvent) {
  match event.menu_item_id() {
    "clear_history" => {
      match clear_operation_history(&event.window().state::<Mutex<Connection>>().lock().unwrap()) {
        Ok(_) => {
          event.window().emit_all("history_cleared", {}).unwrap();
        }
        Err(e) => println!("Error: {}", e),
      }
    }
    _ => {}
  }
}
