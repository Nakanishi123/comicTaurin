// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{self, Engine};
use rand;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::vec;
use tauri::api::file;
use tauri::State;
use walkdir::WalkDir;
use zip::ZipArchive;

struct Book {
    archive: Mutex<Option<ZipArchive<std::fs::File>>>,
    image_indexs: Mutex<Vec<usize>>,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            archive: Mutex::new(None),
            image_indexs: Mutex::new(Vec::new()),
        }
    }
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn get_page(page_index: usize, now_book_state: State<Book>) -> String {
    let mut now_book = now_book_state.archive.lock().unwrap();
    let now_book_image_indexs = now_book_state.image_indexs.lock().unwrap();
    let mut file = now_book
        .as_mut()
        .unwrap()
        .by_index(now_book_image_indexs[page_index])
        .unwrap();
    let mut buffer = Vec::new();
    let n = file.read_to_end(&mut buffer).unwrap();
    let base64_content = base64::engine::general_purpose::STANDARD.encode(&buffer[..n]);
    return format!("data:image/{};base64,{}", "jpeg", base64_content);
}

#[tauri::command]
fn read_zip_book(book_path: String, image_exts: Vec<String>, now_book: State<Book>) -> usize {
    println!("book_path: {}", book_path);
    let file = match std::fs::File::open(book_path) {
        Ok(file) => file,
        Err(e) => {
            println!("error: {}", e);
            return 1000000;
        }
    };
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(_) => return 2000000,
    };
    let mut image_indexs = Vec::new();
    for i in 0..archive.len() {
        let file = match archive.by_index(i) {
            Ok(file) => file,
            Err(_) => continue,
        };
        let file_name = file.name();
        let file_ext = match Path::new(file_name).extension() {
            Some(ext) => ext.to_str().unwrap_or_default(),
            None => continue,
        };
        if image_exts.contains(&file_ext.to_string()) {
            image_indexs.push(i);
        }
    }
    let mut now_book_archive = now_book.archive.lock().unwrap();
    let mut now_book_image_indexs = now_book.image_indexs.lock().unwrap();
    let image_length = image_indexs.len();
    *now_book_archive = Some(archive);
    *now_book_image_indexs = image_indexs;

    return image_length;
}

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

#[derive(Serialize, Deserialize)]
struct PathInfo {
    path: String,
    name: String,
    is_dir: bool,
}

#[tauri::command]
fn get_children(dir_path_str: String) -> Vec<PathInfo> {
    let dir_path = Path::new(&dir_path_str);
    if !dir_path.is_dir() {
        return Vec::new();
    }

    let mut children = Vec::new();
    for entry in dir_path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let is_dir = path.is_dir();
            let path_info = PathInfo {
                path: path.to_string_lossy().into_owned(),
                name: path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                is_dir,
            };
            children.push(path_info);
        }
    }
    children
}

#[tauri::command]
fn get_parent(path_str: String) -> PathInfo {
    let path = Path::new(&path_str);
    if !path.exists() {
        let path_info = PathInfo {
            path: String::new(),
            name: String::new(),
            is_dir: false,
        };
        return path_info;
    }
    match path.parent() {
        Some(parent) => {
            let path_info = PathInfo {
                path: parent.to_str().unwrap().to_string(),
                name: parent
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_string(),
                is_dir: true,
            };
            return path_info;
        }
        None => {
            let path_info = PathInfo {
                path: String::new(),
                name: String::new(),
                is_dir: false,
            };
            return path_info;
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Book::default())
        .invoke_handler(tauri::generate_handler![
            get_randompic,
            get_children,
            get_parent,
            read_zip_book,
            get_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
