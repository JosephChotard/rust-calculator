use super::operations;
use super::system;

pub fn get_handlers() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
  Box::new(tauri::generate_handler![
    system::system_commands::get_system_theme,
    system::system_commands::set_system_theme,
    operations::operations_commands::store_operation_command,
    operations::operations_commands::get_operation_history_command,
    operations::operations_commands::clear_operation_history_command,
    operations::operations_commands::get_result_command,
  ])
}
