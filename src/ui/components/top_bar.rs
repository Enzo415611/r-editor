use iced::{
    Element, Length, Renderer, Theme,
    widget::{self, button, text::Alignment},
};
use iced_aw::{Menu, MenuBar, menu::Item, menu_items};

use crate::{
    GlobalState,
    events::file::FileEvents,
    ui::{style::style::button_style, view::Page},
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn top_bar(&self) -> Element<'_, GlobalMessagens> {
        let menu_bar = MenuBar::new(menu_items!(Item::with_menu(
            button(widget::text("=").align_x(Alignment::Center))
                .padding(0)
                .width(20)
                .style(|t, s| button_style(t, s))
                .on_press(GlobalMessagens::Test),
            Menu::new(vec![
                Item::new(
                    button("Config")
                        .width(Length::Fill)
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalMessagens::UiEvents(
                            crate::events::ui::UiMessages::SwapPage(Page::ConfigPage)
                        ))
                ),
                Item::with_menu(
                    button("File")
                        .width(Length::Fill)
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalMessagens::Test),
                    self.file_menu()
                ),
                Item::new(
                    button("Terminal")
                        .width(Length::Fill)
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalMessagens::UiEvents(
                            crate::events::ui::UiMessages::OpenOrCloseTerm
                        ))
                )
            ])
            .padding(2)
            .spacing(2)
            .width(150)
        )));
        menu_bar.height(22).padding(1).into()
    }

    fn file_menu(&self) -> Menu<'static, GlobalMessagens, Theme, Renderer> {
        let file_menu = Menu::new(vec![
            Item::new(
                button(widget::text("Open File"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalMessagens::File(FileEvents::OpenFile)),
            ),
            Item::new(
                button(widget::text("Open Folder"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalMessagens::File(FileEvents::OpenFolder)),
            ),
            Item::new(
                button(widget::text("Close Folder"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalMessagens::File(FileEvents::CloseFolder)),
            ),
            Item::new(
                button(widget::text("Save"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalMessagens::File(FileEvents::Save)),
            ),
            Item::new(
                button(widget::text(format!(
                    "Auto Save: {}",
                    self.config_state.auto_save_is_active
                )))
                .width(Length::Fill)
                .style(|t, s| button_style(t, s))
                .on_press(GlobalMessagens::File(FileEvents::AutoSave)),
            ),
        ])
        .padding(2)
        .spacing(2)
        .width(150);
        file_menu.into()
    }
}
