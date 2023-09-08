// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine};
use std::{fs::File, io::Read, path::Path, str::Bytes, string};
use zip::ZipArchive;

static IMAGE_EXTS: [&str; 8] = [
    ".apng", ".avif", ".bmp", "gif", "jpg", "jpeg", "png", ".webp",
];

#[tauri::command]
fn get_page_paths(path_str: &str) -> Vec<String> {
    println!("path_str: {}", path_str);
    let path = std::path::Path::new(path_str);
    if ZipArchive::new(std::fs::File::open(path).unwrap()).is_ok() {
        return zip_image_paths(path);
    }

    return vec![];
}

fn zip_image_paths(path: &Path) -> Vec<String> {
    let mut archive = ZipArchive::new(File::open(path).unwrap()).unwrap();
    let mut images: Vec<String> = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let name = file.name();
        if IMAGE_EXTS.iter().any(|&ext| name.ends_with(ext)) {
            images.push(name.to_string());
        }
    }
    images
}

#[tauri::command]
fn get_page(path_str: &str, page: &str) -> String {
    let mut archive = ZipArchive::new(File::open(path_str).unwrap()).unwrap();
    let mut file = match archive.by_name(page) {
        Ok(file) => file,
        Err(_) => return "".to_string(),
    };
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let img_base64 = general_purpose::STANDARD.encode(&buf);
    return format!("data:image/png;base64,{}", img_base64);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_page_paths, get_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
