use iced::{Element, widget::container};

use crate::{State, update::Events};

impl State {
    pub fn file_tree_view(&self) -> Element<'_, Events> {
        container(self.tree.view(Events::Tree)).into()
    }
}
