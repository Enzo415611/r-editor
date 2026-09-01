use iced::{
    Task,
    advanced::widget::{operate, operation::focusable::unfocus},
    widget::{operation::focus_next, pane_grid},
};
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
    OpenOrCloseTerm,
    TerminalCommands(iced_term::Command),
    TerminalEvents(iced_term::Event),
    TerminalEnters,
    TerminalExit,
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
                self.ui_state.tabs.shift_remove(&tab);
                Task::none()
            }
            UiMessages::TerminalEvents(iced_term::Event::BackendCall(id, cmd)) => {
                if let Some(t) = self.ui_state.terminals.get_mut(&id) {
                    t.1.handle(iced_term::Command::ProxyToBackend(cmd));
                }

                Task::none()
            }
            UiMessages::TerminalCommands(c) => {
                _ = self
                    .ui_state
                    .terminals
                    .iter_mut()
                    .map(|t| t.1.1.handle(c.to_owned()));
                Task::none()
            }
            UiMessages::TerminalEnters => {
                self.ui_state.editor.lose_focus();
                focus_next()
            }
            UiMessages::TerminalExit => {
                self.ui_state.editor.request_focus();
                operate(unfocus())
            }
            UiMessages::OpenOrCloseTerm => self.open_terminal_pane(),
        }
    }
}
