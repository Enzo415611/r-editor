use iced::{
    Alignment, Element, Length, Theme,
    widget::{button, column, container, pick_list, row, rule, text},
};

use crate::{
    events::ui::UiMessages,
    state::GlobalState,
    ui::{
        style::style::{button_style, pick_list_style},
        view::Page,
    },
    update::GlobalMessagens,
};

#[derive(Debug, Clone)]
pub enum ConfigSelected {
    Theme(iced::Theme),
}

impl GlobalState {
    pub fn config_page_view(&self) -> Element<'_, GlobalMessagens> {
        let swap_page = button(text("<").align_x(Alignment::Center))
            .padding(0)
            .width(20)
            .style(|t, s| button_style(t, s))
            .on_press(GlobalMessagens::UiEvents(UiMessages::SwapPage(
                Page::EditorPage,
            )));
        let theme = row![
            text("Select Theme: ").height(Length::Fill).center(),
            pick_list(Theme::ALL, self.ui_state.current_theme.clone(), |t| {
                GlobalMessagens::ConfigEvents(ConfigSelected::Theme(t))
            })
            .style(|t, s| pick_list_style(t, s)),
        ]
        .height(30);
        container(column![swap_page, rule::horizontal(1), theme].spacing(3))
            .padding(2)
            .into()
    }
}
