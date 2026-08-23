use iced::{Element, widget::container};

use crate::{State, update::Events};

impl State {
    pub fn editor_view(&self) -> Element<'_, Events> {
        container(self.editor.view().map(Events::Editor)).into()
    }
}
