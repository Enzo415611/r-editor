use std::path::PathBuf;

use iced_code_editor::CodeEditor;

mod file;
mod ui;
mod update;

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view).run()
}

#[derive()]
pub struct State {
    current_path: Option<PathBuf>,
    editor: CodeEditor,
}

impl State {
    fn new() -> Self {
        Self {
            current_path: None,
            editor: CodeEditor::new("", "rs"),
        }
    }
}
