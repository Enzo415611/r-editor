use iced::{
    Element, Length, Theme,
    widget::{button, column, container, pick_list, row, text},
};

use crate::{events::ui::UiMessages, state::GlobalState, ui::view::Page, update::GlobalMessagens};

#[derive(Debug, Clone)]
pub enum ConfigSelected {
    Theme(iced::Theme),
}

impl GlobalState {
    pub fn config_page_view(&self) -> Element<'_, GlobalMessagens> {
        let swap_page = button("<").on_press(GlobalMessagens::UiEvents(UiMessages::SwapPage(
            Page::EditorPage,
        )));
        let theme = row![
            text("Select Theme: ").height(Length::Fill).center(),
            pick_list(Theme::ALL, self.ui_state.current_theme.clone(), |t| {
                GlobalMessagens::ConfigEvents(ConfigSelected::Theme(t))
            })
        ]
        .height(30);
        container(column![swap_page, theme]).into()
    }
}
