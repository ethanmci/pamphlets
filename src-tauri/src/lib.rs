// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::File;
use std::io::prelude::*;

#[tauri::command]
fn save_markdown_file(text: String) -> Result<(), String> {
    let mut file = File::create("test.md").map_err(|err| err.to_string())?;
    file.write_all(text.as_bytes()).map_err(|err| err.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_markdown_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
