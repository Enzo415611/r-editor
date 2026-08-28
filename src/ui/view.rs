use std::path::PathBuf;

use iced::{
    Element, Theme,
    widget::{column, container, pane_grid},
};
use iced_code_editor::CodeEditor;
use iced_swdir_tree::DirectoryTree;

use crate::{State, update::Events};

pub struct UiState {
    pub current_theme: Theme,
    pub editor_grid: pane_grid::State<Pane>,
    pub editor_pane: pane_grid::Pane,
    pub file_tree_pane: pane_grid::Pane,
    pub file_tree_resize: (pane_grid::Split, f32),
    pub file_tree_is_open: bool,
    pub editor: CodeEditor,
    pub tree: DirectoryTree,
}

impl UiState {
    pub fn new() -> Self {
        let (mut state, file_tree_pane) = pane_grid::State::new(Pane::FileTree);
        let (editor_pane, split) = state
            .split(pane_grid::Axis::Vertical, file_tree_pane, Pane::Editor)
            .unwrap();
        let tree = DirectoryTree::new(PathBuf::new())
            .with_filter(iced_swdir_tree::DirectoryFilter::FilesAndFolders);

        let mut editor = CodeEditor::new("", "rs").with_wrap_enabled(false);
        editor.set_theme(iced_code_editor::from_iced_theme(&Theme::CatppuccinMocha));
        Self {
            current_theme: Theme::CatppuccinMocha,
            editor_grid: state,
            file_tree_pane: file_tree_pane,
            file_tree_resize: (split, 0.),
            editor_pane: editor_pane,
            file_tree_is_open: true,
            editor,
            tree,
        }
    }
}

pub enum Pane {
    Editor,
    FileTree,
}

impl State {
    pub fn view(&self) -> Element<'_, Events> {
        let editor_grid = pane_grid(&self.ui_state.editor_grid, |_, state, _| match state {
            Pane::FileTree => container(self.file_tree_view()).into(),
            Pane::Editor => self.editor_view().into(),
        })
        .on_resize(10, Events::ResizeEvent);
        container(column![self.top_bar(), editor_grid,]).into()
    }
}
