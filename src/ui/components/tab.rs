use iced::{
    Color, Element, Length, Theme,
    widget::{button, column, row, rule, scrollable, text},
};

use crate::{events::ui::UiMessages, state::GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn tab_view(&self) -> Element<'static, GlobalMessagens> {
        let r = row![]
            .extend(self.ui_state.tabs.iter().map(|tab| {
                let is_current = self.ui_state.current_tab.as_ref() == Some(tab);

                button(
                    column![row![
                        button(text(format!("{}", tab.tab_name)).center().size(16))
                            .style(move |t, s| button_style(t, s, is_current))
                            .on_press(GlobalMessagens::UiEvents(UiMessages::TabSelected(
                                tab.clone()
                            ),)),
                        button(text("X").size(16).center())
                            .style(move |t, s| button_style(t, s, is_current))
                            .on_press(GlobalMessagens::UiEvents(UiMessages::CloseTab(tab.clone()))),
                    ],]
                    .padding(0)
                    .spacing(1),
                )
                .padding(1)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            }))
            .spacing(3)
            .height(20);
        scrollable(r)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new().width(3).scroller_width(3),
            ))
            .spacing(2)
            .into()
    }
}

fn button_style(t: &Theme, s: button::Status, c: bool) -> button::Style {
    let mut t = button::primary(t, s);
    if c {
        t.text_color = Color::WHITE;
        t.background = Some(iced::Background::Color(Color::from_rgb8(120, 120, 118)));
    }
    t
}

fn rule(is: bool) -> Option<rule::Rule<'static>> {
    if is {
        Some(rule::horizontal(4).style(|t| {
            let mut t = rule::default(t);
            t.color = Color::from_rgb8(0, 255, 198);
            t
        }))
    } else {
        None
    }
}
