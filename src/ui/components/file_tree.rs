use iced::{Element, widget::container};

use crate::{State, update::Events};

impl State {
    pub fn file_tree_view(&self) -> Element<'_, Events> {
        let tree_view = if self.dir_state.current_dir_path.is_none() {
            container("Select Project").into()
        } else {
            container(self.ui_state.tree.view(Events::Tree)).into()
        };

        tree_view
    }
}
