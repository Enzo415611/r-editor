use iced::{
    Element,
    widget::{button, container, row},
};

use crate::{state::GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn terminal_tab_view(&self) -> Element<'static, GlobalMessagens> {
        let terms_button = row![].extend(self.ui_state.terminals.iter().map(|t| button("").into()));
        container(terms_button).into()
    }
}
