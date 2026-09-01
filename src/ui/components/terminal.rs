use iced::{
    Element,
    widget::{column, container, mouse_area, row, rule},
};
use iced_term::Terminal;

use crate::{
    events::ui::UiMessages, state::GlobalState, term::TerminalInfo, update::GlobalMessagens,
};

impl GlobalState {
    pub fn terminal_view(&self) -> Element<'_, GlobalMessagens> {
        let terms: Vec<&(TerminalInfo, Terminal)> = self.ui_state.terminals.values().collect();

        let term_list = row![].extend(terms.iter().map(|t| {
            iced_term::TerminalView::show(&t.1)
                .map(|e| GlobalMessagens::UiEvents(UiMessages::TerminalEvents(e)))
        }));

        mouse_area(container(column![rule::horizontal(1), term_list]))
            .on_enter(GlobalMessagens::UiEvents(UiMessages::TerminalEnters))
            .into()
    }
}
