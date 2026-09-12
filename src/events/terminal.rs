use iced::{
    Task,
    advanced::widget::{operate, operation::focusable::unfocus},
    widget::operation::focus_next,
};

use crate::{state::GlobalState, term::TerminalInfo, update::GlobalMessagens};

#[derive(Debug, Clone)]
pub enum TerminalEvents {
    OpenOrCloseTerm,
    TerminalCommands(iced_term::Command),
    TerminalEvents(iced_term::Event),
    TerminalEnters,
    TerminalExit,
    NewTerminal,
    TerminalTabSelected(TerminalInfo),
    CloseTerminal(u64),
}

impl GlobalState {
    pub fn terminal_events(&mut self, e: TerminalEvents) -> Task<GlobalMessagens> {
        match e {
            TerminalEvents::TerminalEvents(iced_term::Event::BackendCall(id, cmd)) => {
                if let Some(t) = self.ui_state.terminals.get_mut(&id) {
                    t.1.handle(iced_term::Command::ProxyToBackend(cmd));
                }

                Task::none()
            }
            TerminalEvents::TerminalCommands(c) => {
                _ = self
                    .ui_state
                    .terminals
                    .iter_mut()
                    .map(|t| t.1.1.handle(c.to_owned()));
                Task::none()
            }
            TerminalEvents::TerminalEnters => {
                self.ui_state.editor.lose_focus();
                focus_next()
            }
            TerminalEvents::TerminalExit => {
                self.ui_state.editor.request_focus();
                operate(unfocus())
            }
            TerminalEvents::OpenOrCloseTerm => self.open_terminal_pane(),
            TerminalEvents::NewTerminal => {
                let new_term_id = if let Some(last_term) = self.ui_state.terminals.last() {
                    last_term.0 + 1
                } else {
                    0
                };

                let new_term = self.new_terminal(new_term_id);
                self.ui_state.current_terminal = Some(new_term.0.clone());
                self.ui_state.last_terminal = Some(new_term.0.clone());
                self.ui_state.terminals.insert(new_term_id, new_term);
                Task::none()
            }
            TerminalEvents::TerminalTabSelected(t) => {
                self.ui_state.current_terminal = Some(t);
                Task::none()
            }
            TerminalEvents::CloseTerminal(id) => {
                self.ui_state.terminals.shift_remove(&id);

                if self.ui_state.terminals.is_empty() {
                    self.ui_state.current_terminal = None;
                    self.ui_state.last_terminal = None;
                }

                let next_term_tab = self
                    .ui_state
                    .last_terminal
                    .as_ref()
                    .filter(|last| self.ui_state.terminals.contains_key(&last.id))
                    .or_else(|| self.ui_state.terminals.iter().next().map(|(_, t)| &t.0));

                match next_term_tab {
                    Some(t) => {
                        self.ui_state.current_terminal = Some(t.clone());
                        self.ui_state.last_terminal = None;
                    }
                    None => {
                        self.ui_state.current_terminal = None;
                        self.ui_state.last_terminal = None;
                    }
                }

                Task::none()
            }
        }
    }
}
