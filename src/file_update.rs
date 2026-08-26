use std::path::PathBuf;

use iced::Task;
use iced_swdir_tree::DirectoryTree;

use crate::{
    State,
    file::{pick_file, pick_folder, read_file},
    update::Events,
};

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
                self.dir_state.current_file_path = path;
                if let Some(path) = &self.dir_state.current_file_path {
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
                self.dir_state.current_dir_path = path;
                self.tree =
                    DirectoryTree::new(self.dir_state.current_dir_path.clone().unwrap_or_default());
                Task::none()
            }
            FileEvents::Save => Task::none(),
        }
    }
}
