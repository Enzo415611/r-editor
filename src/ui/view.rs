use iced::{
    Element, Length,
    widget::{column, container, mouse_area, pane_grid},
};

use crate::{
    GlobalState,
    events::ui::UiMessages::{self, ResizeEvent},
    update::GlobalMessagens,
};

#[derive(Debug, Clone)]
pub enum Page {
    EditorPage,
    ConfigPage,
}
#[derive(Debug, Clone)]

pub enum Pane {
    Editor,
    FileTree,
    Terminal,
}

impl GlobalState {
    pub fn view(&self) -> Element<'_, GlobalMessagens> {
        let editor_grid = pane_grid(&self.ui_state.editor_grid, |_, state, _| match state {
            Pane::FileTree => container(self.file_tree_view()).into(),
            Pane::Editor => column![self.tab_view(), self.editor_view()].into(),
            Pane::Terminal => mouse_area(container(self.terminal_view()))
                .on_enter(GlobalMessagens::UiEvents(UiMessages::TerminalEnters))
                .into(),
        })
        .width(Length::Fill)
        .on_resize(10, |e| GlobalMessagens::UiEvents(ResizeEvent(e)));
        let editor_page = column![self.top_bar(), editor_grid];

        match self.ui_state.current_page {
            Page::EditorPage => container(editor_page).into(),
            Page::ConfigPage => self.config_page_view(),
        }
    }
}
