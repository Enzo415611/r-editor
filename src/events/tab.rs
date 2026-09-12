use std::path::PathBuf;

use iced::Task;

use crate::{
    events::ui::UiMessages,
    file::read_file,
    state::{GlobalState, Tab},
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn tab_events(&mut self, e: TabEvents) -> Task<GlobalMessagens> {
        match e {
            TabEvents::TabSelected(tab) => {
                self.settings.file_path = tab.path.to_path_buf();
                if self.ui_state.current_file.as_ref() != Some(&tab) {
                    self.ui_state.last_tab = self.ui_state.current_file.clone();
                    self.ui_state.current_file = Some(tab.clone());
                }

                self.dir_state.current_file_path = Some(tab.path.to_path_buf());

                self.ui_state
                    .editor
                    .reset(&read_file(&tab.path).unwrap_or_default())
                    .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)))
            }
            TabEvents::CloseTab(tab) => {
                self.ui_state.tabs.shift_remove(&tab);

                if self.ui_state.tabs.is_empty() {
                    self.ui_state.current_file = None;
                    self.ui_state.last_tab = None;
                    self.dir_state.current_file_path = None;
                    self.settings.file_path = PathBuf::new();
                    return self
                        .ui_state
                        .editor
                        .reset("")
                        .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)));
                }

                let next_tab = self
                    .ui_state
                    .last_tab
                    .as_ref()
                    .filter(|last| self.ui_state.tabs.contains(*last))
                    .cloned()
                    .or_else(|| self.ui_state.tabs.iter().next().cloned());

                match next_tab {
                    Some(tab) => {
                        self.settings.file_path = tab.path.to_path_buf();
                        self.ui_state.current_file = Some(tab.clone());
                        self.ui_state.last_tab = None;
                        self.dir_state.current_file_path = Some(tab.path.to_path_buf());

                        if let Some(content) = read_file(&tab.path) {
                            return self
                                .ui_state
                                .editor
                                .reset(&content)
                                .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)));
                        }
                    }
                    None => {
                        self.ui_state.current_file = None;
                        self.ui_state.last_tab = None;
                        self.dir_state.current_file_path = None;
                        self.settings.file_path = PathBuf::new();

                        return self
                            .ui_state
                            .editor
                            .reset("")
                            .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)));
                    }
                }
                Task::none()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TabEvents {
    TabSelected(Tab),
    CloseTab(Tab),
}
