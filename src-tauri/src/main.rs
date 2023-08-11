// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{self, Engine};
use rand;
use rand::seq::SliceRandom;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::vec;
use tauri::api::file;
use tauri::State;
use walkdir::WalkDir;

fn get_file_paths(dir_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if !dir_path.exists() {
        return Err("Directory does not exist".into());
    }
    // let file_paths: Vec<&PathBuf> = WalkDir::new(dir_path)
    //     .into_iter()
    //     .filter_map(|e| e.ok())
    //     .map(|e| e.path().to_path_buf())
    //     .filter(|path| path.is_file())
    //     .collect::<Vec<&PathBuf>>();

    let mut file_paths = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            file_paths.push(entry.path().to_path_buf());
        }
    }

    return Ok(file_paths);
}

fn get_img_ext(path: &Path) -> Result<&str, &str> {
    let pic_exts: [&OsStr; 5] = [
        OsStr::new("jpg"),
        OsStr::new("jpeg"),
        OsStr::new("png"),
        OsStr::new("webp"),
        OsStr::new("avif"),
    ];

    let ext = path.extension().unwrap_or_default();
    if pic_exts.contains(&ext) {
        return Ok(&ext.to_str().unwrap_or_default());
    } else {
        return Err("Not a picture");
    }
}

fn select_pic_files(file_paths: Vec<&Path>) -> Vec<&Path> {
    return file_paths
        .iter()
        .filter(|path| get_img_ext(path).is_ok())
        .cloned()
        .collect();
}

#[tauri::command]
fn get_randompic() -> String {
    let dir_path = r"H:\User\picture\manga\ア行\あおざくら 防衛大学校物語\25";
    let file_paths = get_file_paths(Path::new(dir_path)).unwrap_or(Vec::new());
    let pic_files = select_pic_files(file_paths.iter().map(|path| path.as_path()).collect());
    let selected_file = pic_files.choose(&mut rand::thread_rng()).unwrap();
    let file_content = file::read_binary(selected_file).unwrap();
    let ext = get_img_ext(selected_file).unwrap();
    let base64_content = base64::engine::general_purpose::STANDARD.encode(&file_content);
    return format!("data:image/{};base64,{}", ext, base64_content);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_randompic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
