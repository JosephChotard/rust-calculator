mod connection;
mod operations;
mod variables;

pub use connection::get_connection;
pub use operations::{clear_operation_history, get_operation_history, store_operation, Operation};
pub use variables::{clear_variables, get_variables, store_variable, Variable};
