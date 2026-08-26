use iced::{Task, keyboard, widget::pane_grid};
use iced_swdir_tree::DirectoryTreeEvent;

use crate::{State, file::read_file, file_update::FileEvents};

#[derive(Debug, Clone)]
pub enum Events {
    Test,
    ResizeEvent(pane_grid::ResizeEvent),
    File(FileEvents),
    Editor(iced_code_editor::Message),
    Tree(DirectoryTreeEvent),
    KeyEvent(keyboard::Event),
}

impl State {
    pub fn update(&mut self, events: Events) -> Task<Events> {
        match events {
            Events::Test => Task::none(),
            Events::ResizeEvent(e) => {
                self.file_tree_resize = (e.split, e.ratio);
                self.editor_grid.resize(e.split, e.ratio);
                Task::none()
            }
            Events::File(e) => self.file_update(e),
            Events::Editor(e) => self.editor_update(e),
            Events::Tree(e) => self.tree_update(e),
            Events::KeyEvent(e) => self.key_event(e),
        }
    }

    pub fn tree_update(&mut self, e: DirectoryTreeEvent) -> Task<Events> {
        match e {
            DirectoryTreeEvent::Selected(path, is_dir, _) => {
                if !is_dir {
                    self.dir_state.current_file_path = Some(path.to_path_buf());

                    if let Some(path) = &self.dir_state.current_file_path {
                        if let Some(content) = read_file(path) {
                            let task = self.editor.reset(&content);
                            return task.map(|event| Events::Editor(event));
                        }
                    }
                }
                Task::none()
            }
            _ => self.tree.update(e).map(Events::Tree),
        }
    }

    pub fn editor_update(&mut self, e: iced_code_editor::Message) -> Task<Events> {
        match e {
            _ => self.editor.update(&e).map(Events::Editor),
        }
    }
}
