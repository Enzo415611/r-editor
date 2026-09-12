use iced::{
    Element,
    widget::{button, row, scrollable, text},
};

use crate::{
    events::{
        terminal::TerminalEvents,
        ui::UiMessages::{self},
    },
    state::GlobalState,
    ui::style::style::tab_button_style,
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn terminal_tab_view(&self) -> Element<'static, GlobalMessagens> {
        let t = row![button(text("New Terminal").center().size(16)).on_press(
            GlobalMessagens::UiEvents(UiMessages::TerminalEvents(TerminalEvents::NewTerminal))
        )]
        .extend(self.ui_state.terminals.iter().map(|t| {
            let term_name = if let Some(p) = &t.1.0.working_dir {
                p.file_name().unwrap_or_default().display().to_string()
            } else {
                format!("{}", t.0)
            };

            let is_current = self.ui_state.current_terminal.as_ref() == Some(&t.1.0);

            row![
                button(text(format!("{}", term_name)).center().size(16))
                    .style(move |t, s| tab_button_style(t, s, is_current))
                    .on_press(GlobalMessagens::UiEvents(UiMessages::TerminalEvents(
                        TerminalEvents::TerminalTabSelected(t.1.0.clone())
                    ))),
                button(text("X").size(16).center())
                    .style(move |t, s| tab_button_style(t, s, is_current))
                    .on_press(GlobalMessagens::UiEvents(UiMessages::TerminalEvents(
                        TerminalEvents::CloseTerminal(*t.0)
                    )))
            ]
            .into()
        }))
        .spacing(3)
        .height(20);

        scrollable(t)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new().width(3).scroller_width(3),
            ))
            .into()
    }
}
