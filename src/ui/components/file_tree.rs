use iced::{
    Alignment, Element, Length,
    widget::{button, container},
};

use crate::{
    GlobalState,
    events::{file::FileEvents, ui::UiMessages},
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn file_tree_view(&self) -> Element<'_, GlobalMessagens> {
        let tree_view = if self.dir_state.current_dir_path.is_none() {
            container(button("Open Folder").on_press(GlobalMessagens::File(FileEvents::OpenFolder)))
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        } else {
            container(
                self.ui_state
                    .tree
                    .view(|e| GlobalMessagens::UiEvents(UiMessages::Tree(e))),
            )
            .into()
        };

        tree_view
    }
}
