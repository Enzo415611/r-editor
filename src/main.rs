use iced::{Subscription, Theme, keyboard};

use crate::{file::DirState, ui::view::UiState, update::Events};

mod events;
mod file;
mod ui;
mod update;

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .theme(|state: &State| state.theme())
        .subscription(|state| subscription(state))
        .run()
}

pub struct State {
    ui_state: UiState,
    dir_state: DirState,
    config_state: ConfigState,
    binds: Binds,
}

pub struct Binds {
    open_file_tree_bind: (keyboard::Modifiers, keyboard::Key),
}

pub struct ConfigState {
    auto_save_is_active: bool,
}

impl ConfigState {
    fn new() -> Self {
        Self {
            auto_save_is_active: false,
        }
    }
}

impl Binds {
    fn new() -> Self {
        Self {
            open_file_tree_bind: (
                keyboard::Modifiers::CTRL,
                keyboard::Key::Character("b".into()),
            ),
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            dir_state: DirState::new(),
            ui_state: UiState::new(),
            config_state: ConfigState::new(),
            binds: Binds::new(),
        }
    }

    fn theme(&self) -> Theme {
        self.ui_state.current_theme.to_owned()
    }
}

fn subscription(_: &State) -> Subscription<Events> {
    let keys = keyboard::listen().map(|e| Events::KeyEvent(e));
    Subscription::batch([keys])
}
