mod env_var;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            env_var::get_env_vars,
            env_var::set_env_var,
            env_var::delete_env_var,
            env_var::check_admin_privileges,
            env_var::request_admin_privileges
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
