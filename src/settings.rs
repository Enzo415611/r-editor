use iced_swdir_tree::DirectoryTree;

use crate::state::{GlobalState, Settings};

impl GlobalState {
    pub fn load_settings(&mut self) -> anyhow::Result<()> {
        let s = confy::load::<Settings>("r-editor", "settings")?;

        self.settings = s;
        // editor theme
        self.ui_state
            .editor
            .set_theme(iced_code_editor::from_iced_theme(
                &self.settings.current_theme.clone().into(),
            ));

        // app theme
        self.ui_state.current_theme = Some(self.settings.current_theme.clone().into());

        // editor wrap
        self.ui_state.editor.set_wrap_enabled(self.settings.wrap);

        // editor font size
        self.ui_state
            .editor
            .set_font_size(self.settings.font_size, true);

        // editor line height
        self.ui_state
            .editor
            .set_line_height(self.settings.line_height);

        // editor line numbers
        self.ui_state
            .editor
            .set_line_numbers_enabled(self.settings.line_numbers);

        // editor vim mode
        self.ui_state.editor.set_vim_enabled(self.settings.vim_mode);

        // dir path
        if !self.settings.dir_path.is_empty() {
            self.dir_state.current_dir_path = Some(self.settings.dir_path.to_path_buf());
        }
        // update file tree with dir path
        if let Some(path) = &self.dir_state.current_dir_path {
            self.ui_state.tree = DirectoryTree::new(path.to_path_buf());
        }

        Ok(())
    }

    // save all settings
    pub fn save_settings(&mut self) -> anyhow::Result<()> {
        confy::store::<Settings>("r-editor", "settings", self.settings.clone())?;
        Ok(())
    }
}
