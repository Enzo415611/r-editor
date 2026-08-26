use iced::{
    Element, Length, Renderer, Theme,
    widget::{button, text},
};
use iced_aw::{Menu, MenuBar, menu::Item, menu_items};

use crate::{State, file_update::FileEvents, update::Events};

impl State {
    pub fn top_bar(&self) -> Element<'_, Events> {
        let menu_bar = MenuBar::new(menu_items!((
            button("File").on_press(Events::Test),
            file_menu()
        )));
        menu_bar.height(30).into()
    }
}

fn file_menu() -> Menu<'static, Events, Theme, Renderer> {
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
    ])
    .padding(0)
    .width(150);
    file_menu.into()
}
