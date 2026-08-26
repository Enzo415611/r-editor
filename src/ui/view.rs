use iced::{
    Element, Length,
    widget::{column, container, pane_grid},
};

use crate::{State, update::Events};

pub enum Pane {
    Editor,
    FileTree,
}

impl State {
    pub fn view(&self) -> Element<'_, Events> {
        let editor_grid = pane_grid(&self.editor_grid, |_, state, _| match state {
            Pane::FileTree => container(self.file_tree_view()).into(),
            Pane::Editor => self.editor_view().into(),
        })
        .on_resize(10, Events::ResizeEvent);
        container(column![self.top_bar(), editor_grid,]).into()
    }
}
