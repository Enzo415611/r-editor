use iced::Task;

use crate::{State, file::FileEvents};

#[derive(Debug, Clone)]
pub enum Events {
    Test,
    File(FileEvents),
    Editor(iced_code_editor::Message),
}

impl State {
    pub fn update(&mut self, events: Events) -> Task<Events> {
        match events {
            Events::Test => Task::none(),
            Events::File(e) => self.file_update(e),
            Events::Editor(event) => match event {
                _ => {
                    let task = self.editor.update(&event);
                    task.map(Events::Editor)
                }
            },
        }
    }
}
