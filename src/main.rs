use iced::{Subscription, keyboard};

use crate::{
    events::{terminal::TerminalEvents, ui::UiMessages, update::GlobalEvents},
    state::GlobalState,
};

mod events;
mod file;
mod settings;
mod state;
mod term;
mod ui;

fn main() -> iced::Result {
    let app = iced::application(GlobalState::new, GlobalState::update, GlobalState::view)
        .title("R Editor")
        .decorations(true)
        .transparent(true)
        .resizable(true)
        .font(iced_swdir_tree::LUCIDE_FONT_BYTES)
        .theme(|state: &GlobalState| state.theme())
        .subscription(GlobalState::subscription);
    app.run()
}

impl GlobalState {
    fn subscription(&self) -> Subscription<GlobalEvents> {
        let keys = keyboard::listen().map(|e| GlobalEvents::KeyEvent(e));
        let term = Subscription::batch(self.ui_state.terminals.iter().map(|t| {
            t.1.1.subscription().map(|e| {
                GlobalEvents::UiEvents(UiMessages::TerminalEvents(TerminalEvents::TerminalEvents(
                    e,
                )))
            })
        }));

        Subscription::batch([keys, term])
    }
}
