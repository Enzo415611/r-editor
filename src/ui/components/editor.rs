use iced::{Element, widget::container};

use crate::{GlobalState, events::ui::UiMessages, update::GlobalMessagens};

impl GlobalState {
    pub fn editor_view(&self) -> Element<'_, GlobalMessagens> {
        container(
            self.ui_state
                .editor
                .view()
                .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e))),
        )
        .padding(2)
        .into()
    }
}
