use iced::{
    Alignment, Element, Length, Theme,
    widget::{button, checkbox, column, container, pick_list, row, rule, text},
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
            text("Select Theme: ").height(Length::Fill).center(),
            pick_list(Theme::ALL, self.ui_state.current_theme.clone(), |t| {
                GlobalMessagens::ConfigEvents(ConfigSelected::Theme(t))
            })
            .style(|t, s| pick_list_style(t, s)),
        ]
        .align_y(Alignment::Center)
        .height(30);

        let vim_mode = row![
            text("Vim Mode")
                .height(Length::Fill)
                .align_y(Alignment::End),
            checkbox(self.settings.vim_mode)
                .text_line_height(1.0)
                .on_toggle(|b| GlobalMessagens::ConfigEvents(ConfigSelected::VimMode(b)))
        ]
        .align_y(Alignment::Center)
        .height(30);

        let font_size = row![
            text("Font Size").height(Length::Fill).center(),
            NumberInput::new(&self.settings.font_size, 3.0..=30.0, |f| {
                GlobalMessagens::ConfigEvents(ConfigSelected::FontSize(f))
            })
            .line_height(1.0)
        ]
        .align_y(Alignment::Center)
        .height(30);

        container(column![swap_page, rule::horizontal(1), theme, vim_mode, font_size].spacing(3))
            .padding(2)
            .into()
    }
}

//  pub vim_mode: bool,
//     pub font_size: f32,
//     pub wrap: bool,
//     pub line_numbers: bool,
//     pub line_height: f32,
