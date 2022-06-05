mod operations;
mod system;

pub fn get_handlers() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
  Box::new(tauri::generate_handler![
    system::get_system_theme,
    system::set_system_theme,
    operations::store_operation_command,
    operations::get_operation_history_command,
    operations::clear_operation_history_command,
    operations::get_result_command,
  ])
}
