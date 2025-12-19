// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri_plugin_store::StoreExt;
use serde_json::json;
use tauri::{Manager, State};

#[derive(Default)]
struct AppState {
    active_file: PathBuf,
}

#[tauri::command]
fn save_markdown_file(text: String) -> Result<(), String> {
    let mut file = File::create("test.md").map_err(|err| err.to_string())?;
    file.write_all(text.as_bytes())
        .map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
fn set_base_dir(state: State<'_, Mutex<AppState>>, dir_string_to_parse: String) -> Result<PathBuf, String> {
    if !Path::new(&dir_string_to_parse).exists() { 
        return Err("Specified path does not exist".to_string());
    } 
    let mut state = state.lock().unwrap();
    state.active_file = PathBuf::from(dir_string_to_parse);
    return Ok(state.active_file.clone());
}

#[tauri::command]
fn get_base_dir(state: State<'_, Mutex<AppState>>) -> PathBuf {
    let state = state.lock().unwrap();
    return state.active_file.clone();
}

#[tauri::command]
async fn save_base_dir() -> Result<(), String> {
  Ok(())
}

#[tauri::command]
async fn dir_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            save_markdown_file,
            set_base_dir,
            get_base_dir,
            dir_exists
        ])
        .setup(|app| {
            let store = app.store("pamphlets_data.json")?;
            //let stored_base_dir = store.get()

            app.manage(Mutex::new(AppState {
                active_file: PathBuf::new(),
            }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
