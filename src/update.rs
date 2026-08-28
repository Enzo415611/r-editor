use iced::{Task, keyboard};
use iced_swdir_tree::DirectoryTreeEvent;

use crate::{
    events::{file::FileEvents, ui::UiMessages},
    file::read_file,
    state::GlobalState,
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
                if let Ok(s) = self.load_settings() {
                    self.ui_state
                        .editor
                        .set_theme(iced_code_editor::from_iced_theme(
                            &self.settings.current_theme.clone().into(),
                        ));
                    self.ui_state.current_theme = Some(self.settings.current_theme.clone().into());
                    self.ui_state.editor.set_wrap_enabled(self.settings.wrap);
                    self.ui_state
                        .editor
                        .set_font_size(self.settings.font_size, true);
                    self.ui_state
                        .editor
                        .set_line_height(self.settings.line_height);
                    self.ui_state
                        .editor
                        .set_line_numbers_enabled(self.settings.line_numbers);
                    self.ui_state.editor.set_vim_enabled(self.settings.vim_mode);
                }
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
                    self.dir_state.current_file_path = Some(path.to_path_buf());

                    if let Some(path) = &self.dir_state.current_file_path {
                        if let Some(content) = read_file(path) {
                            let task = self.ui_state.editor.reset(&content);
                            return task
                                .map(|event| GlobalMessagens::UiEvents(UiMessages::Editor(event)));
                        }
                    }
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
