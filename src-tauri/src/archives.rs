mod static_files;
use base64::engine::general_purpose::STANDARD;
use base64::{self, Engine};
use infer;
use static_files::EMPTY_IMAGE;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;
use zip::ZipArchive;

enum BookType {
    Zip,
    Rar,
    SevenZip,
    Unknown,
}

pub trait Book {
    fn get_page(&mut self, page_number: usize) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct BookState {
    zip_book: Mutex<ZipBook>,
    book_type: Mutex<BookType>,
    image_exts: Mutex<Vec<String>>,
    page_number: Mutex<usize>,
}

impl BookState {
    pub fn default() -> Self {
        Self {
            zip_book: ZipBook::default().into(),
            book_type: BookType::Unknown.into(),
            image_exts: Mutex::new(
                [
                    "apng", "avif", "bmp", "gif", "jpg", "jpeg", "png", "tiff", "webp",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ),
            page_number: 0.into(),
        }
    }

    fn set_book(&self, str_path: String) -> Result<usize, Box<dyn std::error::Error + '_>> {
        if !Path::new(&str_path).exists() {
            return Err("Path does not exist".into());
        }
        // 圧縮形式の判定
        let book_type = match infer::get_from_path(&str_path)? {
            Some(book_type) => match book_type.extension() {
                "zip" => BookType::Zip,
                "rar" => BookType::Rar,
                "7z" => BookType::SevenZip,
                _ => return Err("Not support book type".into()),
            },
            None => return Err("Not support book type".into()),
        };

        let mut book_type_ptr = match self.book_type.lock() {
            Ok(lock) => lock,
            Err(e) => return Err(e.into()),
        };
        let mut page_number_ptr = match self.page_number.lock() {
            Ok(lock) => lock,
            Err(e) => return Err(e.into()),
        };
        let image_exts = match self.image_exts.lock() {
            Ok(lock) => lock.clone(),
            Err(e) => return Err(e.into()),
        };

        let page_number = match book_type {
            BookType::Zip => {
                let mut zip_book_ptr = match self.zip_book.lock() {
                    Ok(lock) => lock,
                    Err(e) => return Err(e.into()),
                };
                let zip_book = match ZipBook::get_book(str_path, image_exts) {
                    Ok(book) => book,
                    Err(e) => return Err(e.into()),
                };
                *zip_book_ptr = zip_book;
                zip_book_ptr.image_indexs.len()
            }
            _ => return Err("Not support book type yet.".into()),
        };

        *book_type_ptr = book_type;
        *page_number_ptr = page_number;

        return Ok(page_number);
    }

    fn get_page(&self, page_number: usize) -> String {
        let book_type = match self.book_type.lock() {
            Ok(lock) => lock,
            Err(_) => return EMPTY_IMAGE.to_string(),
        };
        match *book_type {
            BookType::Zip => {
                let mut zip_book_ptr = match self.zip_book.lock() {
                    Ok(lock) => lock,
                    Err(_) => return EMPTY_IMAGE.to_string(),
                };
                match zip_book_ptr.get_page(page_number) {
                    Ok(page) => page,
                    Err(_) => EMPTY_IMAGE.to_string(),
                }
            }
            _ => EMPTY_IMAGE.to_string(),
        }
    }
}

struct ZipBook {
    archive: Option<ZipArchive<File>>,
    image_indexs: Vec<usize>,
}

impl Book for ZipBook {
    fn get_page(&mut self, page_number: usize) -> Result<String, Box<dyn std::error::Error>> {
        let book = self.archive.as_mut().ok_or("book is None")?;
        let &page_index = self.image_indexs.get(page_number).ok_or("out of index")?;

        let mut file = book.by_index(page_index)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let ext = file.name().split('.').last().ok_or("png")?;
        let base64_content = STANDARD.encode(&buffer);
        Ok(format!("data:image/{};base64,{}", ext, base64_content))
    }
}

impl ZipBook {
    fn default() -> Self {
        Self {
            archive: None,
            image_indexs: Vec::new(),
        }
    }
    fn get_book(
        book_path: String,
        image_exts: Vec<String>,
    ) -> Result<ZipBook, Box<dyn std::error::Error>> {
        let file = File::open(book_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        let mut image_indexs = Vec::new();
        for i in 0..archive.len() {
            let file = match archive.by_index(i) {
                Ok(file) => file,
                Err(_) => continue,
            };
            let file_ext = match Path::new(file.name()).extension() {
                Some(ext) => ext.to_str().unwrap_or_default(),
                None => continue,
            };
            if image_exts.contains(&file_ext.to_string()) {
                image_indexs.push(i);
            }
        }
        Ok(ZipBook {
            archive: Some(archive),
            image_indexs,
        })
    }
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn get_page(page_index: usize, now_book_state: State<BookState>) -> String {
    now_book_state.get_page(page_index)
}

#[tauri::command]
pub fn set_book(path_str: String, now_book_state: State<BookState>) -> Result<usize, String> {
    match now_book_state.set_book(path_str.clone()) {
        Ok(page_number) => Ok(page_number),
        Err(e) => Err(e.to_string()),
    }
}
