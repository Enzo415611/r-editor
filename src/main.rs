use iced::{Subscription, keyboard};

use crate::{state::GlobalState, update::GlobalMessagens};

mod events;
mod file;
mod settings;
mod state;
mod term;
mod ui;
mod update;

fn main() -> iced::Result {
    let app = iced::application(GlobalState::new, GlobalState::update, GlobalState::view)
        .title("R Editor")
        .transparent(true)
        .resizable(true)
        .font(iced_swdir_tree::LUCIDE_FONT_BYTES)
        .theme(|state: &GlobalState| state.theme())
        .subscription(GlobalState::subscription);
    app.run()
}

impl GlobalState {
    fn subscription(&self) -> Subscription<GlobalMessagens> {
        let keys = keyboard::listen().map(|e| GlobalMessagens::KeyEvent(e));
        let term = Subscription::batch(self.ui_state.terminals.iter().map(|t| {
            t.1.subscription()
                .map(|e| GlobalMessagens::UiEvents(events::ui::UiMessages::TerminalEvents(e)))
        }));

        Subscription::batch([keys, term])
    }
}
