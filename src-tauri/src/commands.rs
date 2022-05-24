use tauri::AppHandle;

#[tauri::command]
pub async fn get_system_theme(app: AppHandle) -> String {
  let mode = dark_light::detect();

  match mode {
    dark_light::Mode::Dark => "dark".to_string(),
    dark_light::Mode::Light => "light".to_string(),
  }
}

pub fn get_handlers() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
  Box::new(tauri::generate_handler![get_system_theme])
}
