use iced::{
    Element, Length,
    widget::{column, container, mouse_area, pane_grid, row, rule},
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
            Pane::FileTree => container(row![self.file_tree_view(), rule::vertical(1)]).into(),
            Pane::Editor => row![
                rule::vertical(1),
                column![self.tab_view(), self.editor_view()].padding(1)
            ]
            .into(),
            Pane::Terminal => mouse_area(container(self.terminal_view()))
                .on_enter(GlobalMessagens::UiEvents(UiMessages::TerminalEnters))
                .into(),
        })
        .width(Length::Fill)
        .style(|t| {
            let mut s = pane_grid::default(t);
            s.hovered_split.width = 1.0;
            s.picked_split.width = 1.0;
            s.hovered_region.border = iced::border::color(s.hovered_split.color).width(1.0);
            s
        })
        .on_resize(10, |e| GlobalMessagens::UiEvents(ResizeEvent(e)));
        let editor_page =
            container(column![self.top_bar(), rule::horizontal(1), editor_grid].spacing(3))
                .padding(2);

        match self.ui_state.current_page {
            Page::EditorPage => editor_page.into(),
            Page::ConfigPage => self.config_page_view(),
        }
    }
}
