use iced_swdir_tree::DirectoryTree;

use crate::state::{GlobalState, Settings};

impl GlobalState {
    pub fn load_settings(&mut self) -> anyhow::Result<()> {
        let s = confy::load::<Settings>("r-editor", "settings")?;

        self.settings = s;

        self.ui_state
            .editor
            .set_theme(iced_code_editor::from_iced_theme(
                &self.settings.current_theme.clone().into(),
            ));

        self.ui_state.current_theme = Some(self.settings.current_theme.clone().into());

        self.ui_state.editor.set_wrap_enabled(self.settings.wrap);

        self.ui_state
            .editor
            .set_font_size(self.settings.font_size, true);

        self.ui_state
            .editor
            .set_line_height(self.settings.line_height);

        self.ui_state
            .editor
            .set_line_numbers_enabled(self.settings.line_numbers);

        self.ui_state.editor.set_vim_enabled(self.settings.vim_mode);

        if !self.settings.dir_path.is_empty() {
            self.dir_state.current_dir_path = Some(self.settings.dir_path.to_path_buf());
        }

        if let Some(path) = &self.dir_state.current_dir_path {
            self.ui_state.tree = DirectoryTree::new(path.to_path_buf());
        }

        Ok(())
    }

    pub fn save_settings(&mut self) -> anyhow::Result<()> {
        confy::store::<Settings>("r-editor", "settings", self.settings.clone())?;

        Ok(())
    }
}
