use iced::{
    Alignment, Element, Length, Theme,
    widget::{button, checkbox, column, container, pick_list, row, rule, text, text_input},
};
use iced_aw::NumberInput;

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
    VimMode(bool),
    FontSize(f32),
    Wrap(bool),
    LineNumbers,
    LineHeight,
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
            pick_list(Theme::ALL, self.ui_state.current_theme.clone(), |t| {
                GlobalMessagens::ConfigEvents(ConfigSelected::Theme(t))
            })
            .style(|t, s| pick_list_style(t, s)),
            text("Select Theme: ").height(Length::Fill).center()
        ]
        .spacing(5)
        .align_y(Alignment::Center)
        .height(30);

        let vim_mode = checkbox(self.settings.vim_mode)
            .label("Vim Mode")
            .spacing(5)
            .size(20)
            .style(|t, s| {
                let mut theme = checkbox::primary(t, s);
                theme.border.radius = iced::border::Radius::new(10);
                theme
            })
            .on_toggle(|b| GlobalMessagens::ConfigEvents(ConfigSelected::VimMode(b)));

        let font_size = row![
            NumberInput::new(&self.settings.font_size, 3.0..=30.0, |f| {
                GlobalMessagens::ConfigEvents(ConfigSelected::FontSize(f))
            })
            .ignore_buttons(true)
            .line_height(1.0)
            .input_style(|t, s| {
                let mut theme = text_input::default(t, s);
                theme.border.radius = iced::border::Radius::new(10);
                theme
            }),
            text("Font Size").height(Length::Fill).center(),
        ]
        .align_y(Alignment::Center)
        .spacing(5)
        .height(30);

        container(column![swap_page, rule::horizontal(1), theme, vim_mode, font_size].spacing(7))
            .padding(3)
            .into()
    }
}
