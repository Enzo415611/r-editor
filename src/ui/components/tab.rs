use iced::{
    Element, Length,
    widget::{button, column, row, scrollable, text},
};

use crate::{
    events::{tab::TabEvents, ui::UiMessages},
    state::GlobalState,
    ui::style::style::tab_button_style,
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn tab_view(&self) -> Element<'static, GlobalMessagens> {
        let r = row![]
            .extend(self.ui_state.tabs.iter().map(|tab| {
                let is_current = self.ui_state.current_file.as_ref() == Some(tab);

                button(
                    column![row![
                        button(text(format!("{}", tab.tab_name)).center().size(16))
                            .style(move |t, s| tab_button_style(t, s, is_current))
                            .on_press(GlobalMessagens::UiEvents(UiMessages::TabEvents(
                                TabEvents::TabSelected(tab.clone())
                            ))),
                        button(text("X").size(16).center())
                            .style(move |t, s| tab_button_style(t, s, is_current))
                            .on_press(GlobalMessagens::UiEvents(UiMessages::TabEvents(
                                TabEvents::CloseTab(tab.clone())
                            ))),
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
