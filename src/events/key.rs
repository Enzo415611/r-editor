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
                if modifiers.control() && key.eq(&keyboard::Key::Character("t".into())) {
                    self.open_terminal_pane();
                }

                if modifiers.eq(&self.binds_state.open_file_tree_bind.0)
                    && key.eq(&self.binds_state.open_file_tree_bind.1)
                {
                    self.open_tree_view();
                }

                if modifiers.control()
                    && modifiers.shift()
                    && key.eq(&keyboard::Key::Character("=".into()))
                {
                    self.settings.font_size = self.settings.font_size + 1.0;
                    _ = self.save_settings(self.settings.to_owned());
                    self.ui_state
                        .editor
                        .set_font_size(self.settings.font_size, true);
                }

                if modifiers.control() && key.eq(&keyboard::Key::Character("-".into())) {
                    self.settings.font_size = self.settings.font_size - 1.0;
                    _ = self.save_settings(self.settings.to_owned());
                    self.ui_state
                        .editor
                        .set_font_size(self.settings.font_size, true);
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

    pub fn open_terminal_pane(&mut self) {
        if self.ui_state.terminal_pane_is_open {
            if let Some((_, pane)) = self.ui_state.editor_grid.close(self.ui_state.terminal_pane) {
                self.ui_state.terminal_pane = pane;
                self.ui_state.terminal_pane_is_open = false;
            }
        } else {
            if let Some((terminal_pane, _)) = self.ui_state.editor_grid.split(
                pane_grid::Axis::Horizontal,
                self.ui_state.editor_pane,
                crate::ui::view::Pane::Terminal,
            ) {
                self.ui_state.terminal_pane = terminal_pane;
                self.ui_state.terminal_pane_is_open = true;
                self.ui_state.editor_grid.drop(
                    terminal_pane,
                    pane_grid::Target::Edge(pane_grid::Edge::Bottom),
                );
            }
        }
    }
}
