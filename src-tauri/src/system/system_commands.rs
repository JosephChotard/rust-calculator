use super::Config;
use std::sync::Mutex;
use tauri::State;

/// Returns the dark_mode preference
///
/// Arguments:
///
/// * `config`: This is the global preference state (tauri passses it to the function for us).
///
/// Returns:
///
/// Whether or not the user wants to use dark mode.
#[tauri::command]
pub fn get_system_theme(config: State<Mutex<Config>>) -> bool {
  config.lock().unwrap().dark_mode
}

/// It takes a `State<Mutex<Config>>` and a `bool` and updates the `Config` with the new `dark_mode`
/// value
///
/// Arguments:
///
/// * `config`: This is the global preference state (tauri passes it to the function for us).
/// * `dark_mode`: bool - This is the value that will be passed to the command.
#[tauri::command]
pub fn set_system_theme(config: State<Mutex<Config>>, dark_mode: bool) {
  let mut config = config.lock().unwrap();
  config.update_dark_mode(dark_mode);
}
