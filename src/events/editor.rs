use iced::Task;

use crate::{State, update::Events};

impl State {
    pub fn editor_update(&mut self, e: iced_code_editor::Message) -> Task<Events> {
        match e {
            iced_code_editor::Message::CharacterInput(_) => {
                let r = self.ui_state.editor.update(&e).map(Events::Editor);
                if self.config_state.auto_save_is_active {
                    self.save_file();
                }
                Task::batch(vec![r])
            }
            // ctrl + s pressed ccccvzxl
            iced_code_editor::Message::WriteRequested => {
                self.save_file();
                Task::none()
            }
            _ => self.ui_state.editor.update(&e).map(Events::Editor),
        }
    }
}
