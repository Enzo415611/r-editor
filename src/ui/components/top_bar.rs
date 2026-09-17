use iced::{
    Element, Length, Renderer, Theme,
    widget::{self, button, text::Alignment},
};
use iced_aw::{Menu, MenuBar, menu::Item, menu_items};

use crate::{
    GlobalState,
    events::{file::FileEvents, terminal::TerminalEvents, update::GlobalEvents},
    ui::{style::style::button_style, view::Page},
};

impl GlobalState {
    pub fn top_bar(&self) -> Element<'_, GlobalEvents> {
        let menu_bar = MenuBar::new(menu_items!(Item::with_menu(
            button(widget::text("=").align_x(Alignment::Center))
                .padding(0)
                .width(20)
                .style(|t, s| button_style(t, s))
                .on_press(GlobalEvents::Test),
            Menu::new(vec![
                Item::new(
                    button("Config")
                        .width(Length::Fill)
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalEvents::UiEvents(
                            crate::events::ui::UiMessages::SwapPage(Page::ConfigPage)
                        ))
                ),
                Item::with_menu(
                    button("File")
                        .width(Length::Fill)
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalEvents::Test),
                    self.file_menu()
                ),
                Item::new(
                    button("Terminal")
                        .width(Length::Fill)
                        .style(|t, s| button_style(t, s))
                        .on_press(GlobalEvents::UiEvents(
                            crate::events::ui::UiMessages::TerminalEvents(
                                TerminalEvents::OpenOrCloseTerm
                            )
                        ))
                )
            ])
            .padding(2)
            .spacing(2)
            .width(150)
        )));
        menu_bar.height(22).padding(1).into()
    }

    fn file_menu(&self) -> Menu<'static, GlobalEvents, Theme, Renderer> {
        let file_menu = Menu::new(vec![
            Item::new(
                button(widget::text("Open File"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalEvents::File(FileEvents::OpenFile)),
            ),
            Item::new(
                button(widget::text("Open Folder"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalEvents::File(FileEvents::OpenFolder)),
            ),
            Item::new(
                button(widget::text("Close Folder"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalEvents::File(FileEvents::CloseFolder)),
            ),
            Item::new(
                button(widget::text("Save"))
                    .width(Length::Fill)
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalEvents::File(FileEvents::Save)),
            ),
            Item::new(
                button(widget::text(format!(
                    "Auto Save: {}",
                    self.config_state.auto_save_is_active
                )))
                .width(Length::Fill)
                .style(|t, s| button_style(t, s))
                .on_press(GlobalEvents::File(FileEvents::AutoSave)),
            ),
        ])
        .padding(2)
        .spacing(2)
        .width(150);
        file_menu.into()
    }
}
