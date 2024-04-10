#[cfg(target_os = "windows")]
pub mod windows_connection;

#[cfg(not(target_os = "windows"))]
pub mod unix_connection;