use iced::{
    Alignment, Element, Length, Padding,
    widget::{button, mouse_area, row, scrollable, text},
};

use crate::{
    events::ui::UiMessages, state::GlobalState, ui::style::style::button_style,
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn tab_view(&self) -> Element<'static, GlobalMessagens> {
        let r = row![]
            .extend(self.ui_state.tabs.iter().map(|tab| {
                mouse_area(
                    button(
                        text(format!("{}", tab.tab_name))
                            .size(14)
                            .align_x(Alignment::Center),
                    )
                    .style(|t, s| button_style(t, s))
                    .height(Length::Fill)
                    .padding(Padding::default().horizontal(2))
                    .on_press(GlobalMessagens::UiEvents(
                        UiMessages::TabSelected(tab.clone()),
                    )),
                )
                .on_right_press(GlobalMessagens::UiEvents(UiMessages::CloseTab(tab.clone())))
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
