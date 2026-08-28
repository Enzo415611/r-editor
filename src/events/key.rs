use iced::{Task, keyboard, widget::pane_grid};

use crate::{GlobalState, update::GlobalMessagens};

impl GlobalState {
    pub fn key_update(&mut self, e: keyboard::Event) -> Task<GlobalMessagens> {
        match e {
            keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                modifiers,
                text,
                repeat,
            } => {
                if modifiers == self.binds_state.open_file_tree_bind.0
                    && key == self.binds_state.open_file_tree_bind.1
                {
                    self.open_tree_view();
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn open_tree_view(&mut self) {
        if self.ui_state.file_tree_is_open {
            if let Some((_, pane)) = self
                .ui_state
                .editor_grid
                .close(self.ui_state.file_tree_pane)
            {
                self.ui_state.file_tree_pane = pane;
                self.ui_state.file_tree_is_open = false;
            }
        } else {
            if let Some((tree_pane, new_split)) = self.ui_state.editor_grid.split(
                pane_grid::Axis::Vertical,
                self.ui_state.file_tree_pane,
                crate::ui::view::Pane::FileTree,
            ) {
                self.ui_state.file_tree_resize.0 = new_split;
                self.ui_state.file_tree_pane = tree_pane;
                self.ui_state
                    .editor_grid
                    .swap(self.ui_state.editor_pane, self.ui_state.file_tree_pane);
                self.ui_state.file_tree_is_open = true;
                self.ui_state.editor_grid.resize(
                    self.ui_state.file_tree_resize.0,
                    self.ui_state.file_tree_resize.1,
                );
            }
        }
    }
}
