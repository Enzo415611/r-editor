use iced::{Element, widget::container};

use crate::{State, update::Events};

impl State {
    pub fn editor_view(&self) -> Element<'_, Events> {
        container(self.ui_state.editor.view().map(Events::Editor))
            .padding(2)
            .into()
    }
}
