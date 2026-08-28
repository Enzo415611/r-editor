use iced::Task;

use crate::{GlobalState, events::ui::UiMessages, update::GlobalMessagens};
use iced_code_editor::Message as EditorMessage;

impl GlobalState {
    pub fn editor_update(&mut self, e: iced_code_editor::Message) -> Task<GlobalMessagens> {
        match e {
            EditorMessage::CharacterInput(_)
            | EditorMessage::Delete
            | EditorMessage::Tab
            | EditorMessage::Backspace
            | EditorMessage::Redo
            | EditorMessage::Undo
            | EditorMessage::Cut
            | EditorMessage::Paste(_) => {
                let r = self
                    .ui_state
                    .editor
                    .update(&e)
                    .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)));
                if self.config_state.auto_save_is_active {
                    self.save_file();
                }
                Task::batch(vec![r])
            }
            // ctrl + s pressed
            iced_code_editor::Message::WriteRequested => {
                self.save_file();
                Task::none()
            }
            _ => self
                .ui_state
                .editor
                .update(&e)
                .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e))),
        }
    }
}
