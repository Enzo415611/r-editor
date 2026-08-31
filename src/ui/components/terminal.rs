use iced::{
    Element,
    widget::{column, container, mouse_area, rule},
};

use crate::{events::ui::UiMessages, state::GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn terminal_view(&self) -> Element<'_, GlobalMessagens> {
        mouse_area(container(column![
            rule::horizontal(1),
            iced_term::TerminalView::show(&self.ui_state.terminals.get(&u64::from(0)))
                .map(|e| { GlobalMessagens::UiEvents(UiMessages::TerminalEvents(e)) }),
        ]))
        .on_enter(GlobalMessagens::UiEvents(UiMessages::TerminalEnters))
        .into()
    }
}
