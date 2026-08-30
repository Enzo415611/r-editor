use iced::{Element, widget::container};

use crate::{state::GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn terminal_view(&self) -> Element<'_, GlobalMessagens> {
        container(
            iced_term::TerminalView::show(&self.ui_state.terminal).map(|e| {
                GlobalMessagens::UiEvents(crate::events::ui::UiMessages::TerminalEvents(e))
            }),
        )
        .into()
    }
}
