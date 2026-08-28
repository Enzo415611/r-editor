use std::{fs, path::PathBuf};

use crate::GlobalState;

impl GlobalState {
    pub fn save_file(&mut self) {
        if let Some(path) = &self.dir_state.current_file_path {
            if let Err(err) = fs::write(path, self.ui_state.editor.content()) {
                eprintln!("{}", err);
            }
        }
    }
}

pub async fn pick_file() -> Option<PathBuf> {
    if let Some(handle) = rfd::AsyncFileDialog::new().pick_file().await {
        return Some(handle.path().to_path_buf());
    }
    None
}

pub async fn pick_folder() -> Option<PathBuf> {
    if let Some(handle) = rfd::AsyncFileDialog::new().pick_folder().await {
        return Some(handle.path().to_path_buf());
    }
    None
}

pub fn read_file(path: &PathBuf) -> Option<String> {
    if let Ok(content) = fs::read_to_string(path) {
        return Some(content);
    }
    None
}
