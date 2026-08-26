use std::path::PathBuf;

use iced::{Subscription, keyboard, widget::pane_grid};
use iced_code_editor::CodeEditor;
use iced_swdir_tree::DirectoryTree;

use crate::{file::DirState, ui::view::Pane, update::Events};

mod file;
mod file_update;
mod key_event;
mod ui;
mod update;

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .subscription(|state| subscription(state))
        .run()
}

pub struct State {
    dir_state: DirState,
    editor_grid: pane_grid::State<Pane>,
    editor_pane: pane_grid::Pane,
    file_tree_pane: pane_grid::Pane,
    file_tree_resize: (pane_grid::Split, f32),
    file_tree_is_open: bool,
    editor: CodeEditor,
    tree: DirectoryTree,
    binds: Binds,
}

pub struct Binds {
    open_file_tree_bind: (keyboard::Modifiers, keyboard::Key),
}

impl Binds {
    fn new() -> Self {
        Self {
            open_file_tree_bind: (
                keyboard::Modifiers::CTRL,
                keyboard::Key::Character("b".into()),
            ),
        }
    }
}

impl State {
    fn new() -> Self {
        let (mut state, file_tree_pane) = pane_grid::State::new(Pane::FileTree);
        let (editor_pane, split) = state
            .split(pane_grid::Axis::Vertical, file_tree_pane, Pane::Editor)
            .unwrap();
        let tree = DirectoryTree::new(PathBuf::new())
            .with_filter(iced_swdir_tree::DirectoryFilter::FilesAndFolders);
        Self {
            dir_state: DirState::new(),
            editor_grid: state,
            file_tree_pane: file_tree_pane,
            file_tree_resize: (split, 0.),
            editor_pane: editor_pane,
            file_tree_is_open: true,
            editor: CodeEditor::new("", "rs"),
            tree,
            binds: Binds::new(),
        }
    }
}

fn subscription(_: &State) -> Subscription<Events> {
    let keys = keyboard::listen().map(|e| Events::KeyEvent(e));
    Subscription::batch([keys])
}
