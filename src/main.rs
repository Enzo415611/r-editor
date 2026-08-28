use iced::{Subscription, keyboard};

use crate::{state::GlobalState, update::GlobalMessagens};

mod events;
mod file;
mod settings;
mod state;
mod ui;
mod update;
fn main() -> iced::Result {
    iced::application(GlobalState::new, GlobalState::update, GlobalState::view)
        .resizable(true)
        .font(iced_swdir_tree::LUCIDE_FONT_BYTES)
        .theme(|state: &GlobalState| state.theme())
        .subscription(|state| subscription(state))
        .run()
}

fn subscription(_: &GlobalState) -> Subscription<GlobalMessagens> {
    let keys = keyboard::listen().map(|e| GlobalMessagens::KeyEvent(e));
    Subscription::batch([keys])
}
