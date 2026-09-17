use std::fmt::Debug;

use iced::{Task, keyboard};
use iced_swdir_tree::DirectoryTreeEvent;

use crate::{
    events::{file::FileEvents, ui::UiMessages},
    file::read_file,
    state::{GlobalState, Tab},
    ui::config_page::ConfigSelected,
};

#[derive(Debug, Clone)]
pub enum GlobalEvents {
    InitConfig,
    Test,
    UiEvents(UiMessages),
    File(FileEvents),
    KeyEvent(keyboard::Event),
    ConfigEvents(ConfigSelected),
}

impl GlobalState {
    pub fn update(&mut self, events: GlobalEvents) -> Task<GlobalEvents> {
        match events {
            GlobalEvents::InitConfig => {
                _ = self.load_settings();

                if let Some(path) = self.dir_state.current_dir_path.to_owned() {
                    return self
                        .ui_state
                        .tree
                        .update(iced_swdir_tree::DirectoryTreeEvent::Toggled(path))
                        .map(|e| GlobalEvents::UiEvents(UiMessages::Tree(e)));
                }

                Task::none()
            }
            GlobalEvents::Test => Task::none(),
            GlobalEvents::File(e) => self.file_update(e),
            GlobalEvents::KeyEvent(e) => self.key_update(e),
            GlobalEvents::ConfigEvents(e) => self.config_update(e),
            GlobalEvents::UiEvents(e) => self.ui_events(e),
        }
    }

    pub fn tree_update(&mut self, e: DirectoryTreeEvent) -> Task<GlobalEvents> {
        match e {
            DirectoryTreeEvent::Selected(path, is_dir, _) => {
                if !is_dir {
                    // if false is file
                    self.dir_state.current_file_path = Some(path.to_path_buf());
                    self.settings.file_path = path.to_path_buf();
                    let name = path.file_name().unwrap_or_default().display().to_string();
                    let tab = Tab {
                        tab_name: name,
                        path: path,
                    };

                    self.ui_state.tabs.insert(tab.clone());
                    if self.ui_state.last_tab.is_none() {
                        if self.ui_state.tabs.len() > 1 {
                            self.ui_state.last_tab = Some(tab.clone())
                        }
                    }
                    self.ui_state.current_file = Some(tab.clone());

                    _ = self.save_settings();

                    if let Some(path) = &self.dir_state.current_file_path {
                        if let Some(content) = read_file(path) {
                            let task = self.ui_state.editor.reset(&content);
                            return task
                                .map(|event| GlobalEvents::UiEvents(UiMessages::Editor(event)));
                        }
                    }
                }
                Task::none()
            }
            _ => self
                .ui_state
                .tree
                .update(e)
                .map(|e| GlobalEvents::UiEvents(UiMessages::Tree(e))),
        }
    }
}
