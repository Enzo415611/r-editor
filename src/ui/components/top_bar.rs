use iced::{
    Element, Length, Renderer, Theme,
    widget::{button, text},
};
use iced_aw::{Menu, MenuBar, menu::Item, menu_items};

use crate::{State, events::file::FileEvents, update::Events};

impl State {
    pub fn top_bar(&self) -> Element<'_, Events> {
        let menu_bar = MenuBar::new(menu_items!(Item::with_menu(
            button("=").on_press(Events::Test),
            Menu::new(vec![
                Item::new(button("Config").width(Length::Fill).on_press(Events::Test)),
                Item::with_menu(
                    button("File").width(Length::Fill).on_press(Events::Test),
                    self.file_menu()
                )
            ])
            .padding(0)
            .width(150)
        )));
        menu_bar.height(30).into()
    }

    fn file_menu(&self) -> Menu<'static, Events, Theme, Renderer> {
        let file_menu = Menu::new(vec![
            Item::new(
                button(text("Open File"))
                    .width(Length::Fill)
                    .on_press(Events::File(FileEvents::OpenFile)),
            ),
            Item::new(
                button(text("Open Folder"))
                    .width(Length::Fill)
                    .on_press(Events::File(FileEvents::OpenFolder)),
            ),
            Item::new(
                button(text("Save"))
                    .width(Length::Fill)
                    .on_press(Events::File(FileEvents::Save)),
            ),
            Item::new(
                button(text(format!(
                    "Auto Save: {}",
                    self.config_state.auto_save_is_active
                )))
                .width(Length::Fill)
                .on_press(Events::File(FileEvents::AutoSave)),
            ),
        ])
        .padding(0)
        .width(150);
        file_menu.into()
    }
}
