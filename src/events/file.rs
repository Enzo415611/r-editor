use std::path::PathBuf;

use iced::Task;
use iced_swdir_tree::DirectoryTree;

use crate::{
    GlobalState,
    file::{pick_file, pick_folder, read_file},
    update::GlobalMessagens,
};

#[derive(Debug, Clone)]
pub enum FileEvents {
    OpenFile,
    OpenFileLoaded(Option<PathBuf>),
    OpenFolder,
    OpenFolderLoaded(Option<PathBuf>),
    CloseFolder,
    Save,
    AutoSave,
}

impl GlobalState {
    pub fn file_update(&mut self, events: FileEvents) -> Task<GlobalMessagens> {
        match events {
            FileEvents::OpenFile => Task::perform(pick_file(), |r| {
                GlobalMessagens::File(FileEvents::OpenFileLoaded(r))
            }),
            FileEvents::OpenFileLoaded(path) => {
                self.dir_state.current_file_path = path;
                if let Some(path) = &self.dir_state.current_file_path {
                    if let Some(content) = read_file(path) {
                        let task = self.ui_state.editor.reset(&content);
                        return task.map(|event| {
                            GlobalMessagens::UiEvents(super::ui::UiMessages::Editor(event))
                        });
                    }
                }
                Task::none()
            }
            FileEvents::OpenFolder => Task::perform(pick_folder(), |r| {
                GlobalMessagens::File(FileEvents::OpenFolderLoaded(r))
            }),
            FileEvents::OpenFolderLoaded(path) => {
                if let Some(path) = &path {
                    self.settings.dir_path = path.to_path_buf();
                    if let Err(err) = self.save_settings() {
                        eprintln!("{}", err)
                    }
                }
                self.dir_state.current_dir_path = path;

                self.ui_state.tree =
                    DirectoryTree::new(self.dir_state.current_dir_path.clone().unwrap_or_default());
                Task::none()
            }
            FileEvents::CloseFolder => {
                if self.dir_state.current_dir_path.is_some() {
                    self.ui_state.tabs.clear();
                    self.dir_state.current_dir_path = None;
                    self.settings.dir_path = PathBuf::new();
                    if let Err(err) = self.save_settings() {
                        eprintln!("{}", err)
                    }
                    return self
                        .ui_state
                        .editor
                        .reset("")
                        .map(|e| GlobalMessagens::UiEvents(super::ui::UiMessages::Editor(e)));
                }
                Task::none()
            }
            FileEvents::Save => {
                self.save_file();
                Task::none()
            }
            FileEvents::AutoSave => {
                self.config_state.auto_save_is_active = !self.config_state.auto_save_is_active;
                Task::none()
            }
        }
    }
}
