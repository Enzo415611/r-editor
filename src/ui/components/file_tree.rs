use iced::{
    Alignment, Element, Length,
    widget::{button, container},
};

use crate::{
    GlobalState,
    events::{file::FileEvents, ui::UiMessages, update::GlobalEvents},
    ui::style::style::button_style,
};

impl GlobalState {
    pub fn file_tree_view(&self) -> Element<'_, GlobalEvents> {
        let tree_view = if self.dir_state.current_dir_path.is_none() {
            container(
                button("Open Folder")
                    .style(|t, s| button_style(t, s))
                    .on_press(GlobalEvents::File(FileEvents::OpenFolder)),
            )
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        } else {
            container(
                self.ui_state
                    .tree
                    .view(|e| GlobalEvents::UiEvents(UiMessages::Tree(e))),
            )
            .into()
        };

        tree_view
    }
}
