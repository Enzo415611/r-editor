use iced::Element;

use crate::{GlobalState, events::ui::UiMessages, update::GlobalMessagens};

impl GlobalState {
    pub fn editor_view(&self) -> Element<'_, GlobalMessagens> {
        self.ui_state
            .editor
            .view()
            .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)))
    }
}
