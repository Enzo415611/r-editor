use iced::Element;

use crate::{
    GlobalState,
    events::{ui::UiMessages, update::GlobalEvents},
};

impl GlobalState {
    pub fn editor_view(&self) -> Element<'_, GlobalEvents> {
        self.ui_state
            .editor
            .view()
            .map(|e| GlobalEvents::UiEvents(UiMessages::Editor(e)))
    }
}
