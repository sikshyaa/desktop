#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use sikshyaa_core::SikshyaaApp;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let database_path = app_data_dir.join("sikshyaa.db");
            let sikshyaa_app = tauri::async_runtime::block_on(
                SikshyaaApp::with_file_surreal(database_path),
            )?;

            app.manage(sikshyaa_app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_formats_name_correctly() {
        assert_eq!(greet("momo"), "Hello, momo! You've been greeted from Rust!");
    }

    #[test]
    fn greet_handles_empty_name() {
        assert_eq!(greet(""), "Hello, ! You've been greeted from Rust!");
    }
}
