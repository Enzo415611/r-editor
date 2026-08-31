use iced::{
    Alignment, Element,
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
                    button(text(format!("{}", tab.tab_name)).align_y(Alignment::Center))
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalMessagens::UiEvents(UiMessages::TabSelected(
                            tab.clone(),
                        ))),
                )
                .on_right_press(GlobalMessagens::UiEvents(UiMessages::CloseTab(tab.clone())))
                .into()
            }))
            .spacing(3)
            .height(24);
        scrollable(r)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::hidden(),
            ))
            .into()
    }
}
