use std::{
    fs::{self},
    path::PathBuf,
};

use iced::Task;

use crate::{State, update::Events};

#[derive(Debug, Clone)]
pub enum FileEvents {
    OpenFile,
    OpenFileLoaded(Option<PathBuf>),
    OpenFolder,
    OpenFolderLoaded(Option<PathBuf>),
    Save,
}

impl State {
    pub fn file_update(&mut self, events: FileEvents) -> Task<Events> {
        match events {
            FileEvents::OpenFile => {
                Task::perform(pick_file(), |r| Events::File(FileEvents::OpenFileLoaded(r)))
            }
            FileEvents::OpenFileLoaded(path) => {
                self.current_path = path;
                if let Some(path) = &self.current_path {
                    if let Some(content) = read_file(path) {
                        let task = self.editor.reset(&content);
                        return task.map(|event| Events::Editor(event));
                    }
                }
                Task::none()
            }
            FileEvents::OpenFolder => Task::perform(pick_folder(), |r| {
                Events::File(FileEvents::OpenFolderLoaded(r))
            }),
            FileEvents::OpenFolderLoaded(path) => {
                self.current_path = path;
                Task::none()
            }
            FileEvents::Save => Task::none(),
        }
    }

    pub fn save_file(&self) {
        if let Some(path) = &self.current_path {
            if let Err(err) = fs::write(path, self.editor.content()) {
                eprintln!("{}", err);
            }
        }
    }
}

async fn pick_file() -> Option<PathBuf> {
    if let Some(handle) = rfd::AsyncFileDialog::new().pick_file().await {
        return Some(handle.path().to_path_buf());
    }
    None
}

async fn pick_folder() -> Option<PathBuf> {
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
