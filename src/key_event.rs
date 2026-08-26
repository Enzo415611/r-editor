use iced::{Task, keyboard, widget::pane_grid};

use crate::{State, update::Events};

impl State {
    pub fn key_event(&mut self, e: keyboard::Event) -> Task<Events> {
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
                if modifiers == self.binds.open_file_tree_bind.0
                    && key == self.binds.open_file_tree_bind.1
                {
                    if self.file_tree_is_open {
                        if let Some((_, pane)) = self.editor_grid.close(self.file_tree_pane) {
                            self.file_tree_pane = pane;
                            self.file_tree_is_open = false;
                        }
                    } else {
                        if let Some((tree_pane, new_split)) = self.editor_grid.split(
                            pane_grid::Axis::Vertical,
                            self.file_tree_pane,
                            crate::ui::view::Pane::FileTree,
                        ) {
                            self.file_tree_resize.0 = new_split;
                            self.file_tree_pane = tree_pane;
                            self.editor_grid.swap(self.editor_pane, self.file_tree_pane);
                            self.file_tree_is_open = true;
                            self.editor_grid
                                .resize(self.file_tree_resize.0, self.file_tree_resize.1);
                        }
                    }
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
