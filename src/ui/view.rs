use iced::{
    Element,
    widget::{column, container},
};

use crate::{State, update::Events};

impl State {
    pub fn view(&self) -> Element<'_, Events> {
        container(column![self.top_bar(), self.editor_view()]).into()
    }
}
