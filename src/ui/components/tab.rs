use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, container, mouse_area, row, scrollable, text},
};

use crate::{events::ui::UiMessages, state::GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn tab_view(&self) -> Element<'static, GlobalMessagens> {
        let r = row![]
            .extend(self.ui_state.tabs.iter().map(|tab| {
                button(
                    row![
                        button(text(format!("{}", tab.tab_name)).center().size(13)).on_press(
                            GlobalMessagens::UiEvents(UiMessages::TabSelected(tab.clone()),)
                        ),
                        button(text("X").size(13).center())
                            .on_press(GlobalMessagens::UiEvents(UiMessages::CloseTab(tab.clone())))
                    ]
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .padding(0)
                    .spacing(1),
                )
                .padding(2)
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

fn tab_button() -> Element<'static, GlobalMessagens> {
    button(row![button(text("zzz").size(14)), button(text("X").size(14))].spacing(1))
        .padding(2)
        .into()
}
