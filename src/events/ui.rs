use std::fmt::Debug;

use iced::{Task, widget::pane_grid};
use iced_swdir_tree::DirectoryTreeEvent;

use crate::{
    events::{tab::TabEvents, terminal::TerminalEvents},
    state::GlobalState,
    ui::view::Page,
    update::GlobalMessagens,
};

#[derive(Debug, Clone)]
pub enum UiMessages {
    SwapPage(Page),
    ResizeEvent(pane_grid::ResizeEvent),
    Tree(DirectoryTreeEvent),
    Editor(iced_code_editor::Message),
    TerminalEvents(TerminalEvents),
    TabEvents(TabEvents),
}

impl GlobalState {
    pub fn ui_events(&mut self, e: UiMessages) -> Task<GlobalMessagens> {
        match e {
            UiMessages::SwapPage(page) => {
                self.ui_state.current_page = page;
                Task::none()
            }
            UiMessages::ResizeEvent(e) => {
                self.ui_state.file_tree_resize = (e.split, e.ratio);
                self.ui_state.editor_grid.resize(e.split, e.ratio);
                Task::none()
            }
            UiMessages::Tree(e) => self.tree_update(e),
            UiMessages::Editor(e) => self.editor_update(e),
            UiMessages::TerminalEvents(e) => self.terminal_events(e),
            UiMessages::TabEvents(e) => self.tab_events(e),
        }
    }
}
