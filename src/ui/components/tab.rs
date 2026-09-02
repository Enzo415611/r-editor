use iced::{
    Background, Color, Element, Length,
    widget::{button, column, row, rule, scrollable, text},
};

use crate::{events::ui::UiMessages, state::GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn tab_view(&self) -> Element<'static, GlobalMessagens> {
        let r = row![]
            .extend(self.ui_state.tabs.iter().map(|tab| {
                let is_current = self.ui_state.current_tab.as_ref() == Some(tab);

                button(
                    row![
                        rule(is_current),
                        button(text(format!("{}", tab.tab_name)).center().size(16)).on_press(
                            GlobalMessagens::UiEvents(UiMessages::TabSelected(tab.clone()),)
                        ),
                        button(text("X").size(16).center())
                            .on_press(GlobalMessagens::UiEvents(UiMessages::CloseTab(tab.clone()))),
                        rule(is_current),
                    ]
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
fn rule(is: bool) -> Option<rule::Rule<'static>> {
    if is { Some(rule::vertical(1)) } else { None }
}
