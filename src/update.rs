use iced::{Task, keyboard};
use iced_swdir_tree::DirectoryTreeEvent;

use crate::{
    events::{file::FileEvents, ui::UiMessages},
    file::read_file,
    state::{GlobalState, Tab},
    ui::config_page::ConfigSelected,
};

#[derive(Debug, Clone)]
pub enum GlobalMessagens {
    InitConfig,
    Test,
    UiEvents(UiMessages),
    File(FileEvents),
    KeyEvent(keyboard::Event),
    ConfigEvents(ConfigSelected),
}

impl GlobalState {
    pub fn update(&mut self, events: GlobalMessagens) -> Task<GlobalMessagens> {
        match events {
            GlobalMessagens::InitConfig => {
                _ = self.load_settings();
                Task::none()
            }
            GlobalMessagens::Test => Task::none(),
            GlobalMessagens::File(e) => self.file_update(e),
            GlobalMessagens::KeyEvent(e) => self.key_update(e),
            GlobalMessagens::ConfigEvents(e) => self.config_update(e),
            GlobalMessagens::UiEvents(e) => self.ui_events(e),
        }
    }

    pub fn tree_update(&mut self, e: DirectoryTreeEvent) -> Task<GlobalMessagens> {
        match e {
            DirectoryTreeEvent::Selected(path, is_dir, _) => {
                if !is_dir {
                    // if false is file
                    self.dir_state.current_file_path = Some(path.to_path_buf());

                    let name = path.file_name().unwrap_or_default().display().to_string();
                    let tab = Tab {
                        tab_name: name,
                        path: path,
                    };

                    if self.ui_state.current_tab.as_ref() != Some(&tab) {
                        self.ui_state.last_tab = self.ui_state.current_tab.clone();
                        self.ui_state.current_tab = Some(tab.clone());
                    }

                    self.ui_state.tabs.insert(tab.clone());

                    if let Some(path) = &self.dir_state.current_file_path {
                        if let Some(content) = read_file(path) {
                            let task = self.ui_state.editor.reset(&content);
                            return task
                                .map(|event| GlobalMessagens::UiEvents(UiMessages::Editor(event)));
                        }
                    }

                    println!("u: c: {:?}", self.ui_state.current_tab);
                    println!("U: l: {:?}", self.ui_state.last_tab);
                }
                Task::none()
            }
            _ => self
                .ui_state
                .tree
                .update(e)
                .map(|e| GlobalMessagens::UiEvents(UiMessages::Tree(e))),
        }
    }
}
