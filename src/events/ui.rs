use iced::{Task, widget::pane_grid};
use iced_swdir_tree::DirectoryTreeEvent;

use crate::{
    file::read_file,
    state::{GlobalState, Tab},
    ui::view::Page,
    update::GlobalMessagens,
};

#[derive(Debug, Clone)]
pub enum UiMessages {
    SwapPage(Page),
    ResizeEvent(pane_grid::ResizeEvent),
    Tree(DirectoryTreeEvent),
    Editor(iced_code_editor::Message),
    TabSelected(Tab),
    CloseTab(Tab),
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
            UiMessages::TabSelected(tab) => {
                if let Some(content) = read_file(&tab.path) {
                    return self
                        .ui_state
                        .editor
                        .reset(&content)
                        .map(|e| GlobalMessagens::UiEvents(UiMessages::Editor(e)));
                }
                self.dir_state.current_file_path = Some(tab.path);
                Task::none()
            }
            UiMessages::CloseTab(tab) => {
                self.ui_state.tabs.remove(&tab);
                Task::none()
            }
        }
    }
}
