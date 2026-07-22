#[tauri::command]
fn greet(name: &str) -> String {
    tracing::debug!(name, "greet command invoked");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use sikshyaa_core::SikshyaaApp;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load local development settings before reading RUST_LOG.
    let _ = dotenvy::dotenv();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new("desktop_lib=debug,sikshyaa_core=debug,surrealdb=info")
    });

    // `try_init` keeps startup safe if another integration already installed a subscriber.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();

    tracing::info!("starting Sikshyaa desktop application");

    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let database_path = app_data_dir.join("sikshyaa.db");
            tracing::info!(path = ?database_path, "initializing SurrealDB");
            let sikshyaa_app =
                tauri::async_runtime::block_on(SikshyaaApp::with_file_surreal(&database_path))?;

            app.manage(sikshyaa_app);
            tracing::info!("SurrealDB initialized successfully");
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
