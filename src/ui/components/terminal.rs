use iced::{
    Element,
    widget::{column, container, mouse_area, rule},
};

use crate::{
    events::{terminal::TerminalEvents, ui::UiMessages},
    state::GlobalState,
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn terminal_view(&self) -> Element<'_, GlobalMessagens> {
        let term = if let Some(t) = &self.ui_state.current_terminal {
            if let Some((_, t)) = self.ui_state.terminals.get(&t.id) {
                Some(iced_term::TerminalView::show(t).map(|e| {
                    GlobalMessagens::UiEvents(UiMessages::TerminalEvents(
                        TerminalEvents::TerminalEvents(e),
                    ))
                }))
            } else {
                None
            }
        } else {
            None
        };

        mouse_area(container(column![
            rule::horizontal(1),
            self.terminal_tab_view(),
            term
        ]))
        .on_enter(GlobalMessagens::UiEvents(UiMessages::TerminalEvents(
            TerminalEvents::TerminalEnters,
        )))
        .into()
    }
}
