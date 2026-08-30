use std::{collections::HashSet, path::PathBuf};

use iced::{Task, Theme, keyboard, widget::pane_grid};
use iced_code_editor::CodeEditor;
use iced_swdir_tree::DirectoryTree;
use serde::{Deserialize, Serialize};

use crate::{
    ui::view::{Page, Pane},
    update::GlobalMessagens,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub current_theme: AppTheme,
    pub vim_mode: bool,
    pub font_size: f32,
    pub wrap: bool,
    pub line_numbers: bool,
    pub line_height: f32,
    pub dir_path: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            current_theme: AppTheme::CatppuccinMocha,
            vim_mode: false,
            font_size: 14.0,
            wrap: false,
            line_numbers: true,
            line_height: 20.0,
            dir_path: PathBuf::new(),
        }
    }
}

pub struct GlobalState {
    pub ui_state: UiState,
    pub dir_state: DirState,
    pub config_state: ConfigState,
    pub binds_state: BindsState,
    pub settings: Settings,
}

impl GlobalState {
    pub fn new() -> (Self, Task<GlobalMessagens>) {
        (
            Self {
                dir_state: DirState::new(),
                ui_state: UiState::new(),
                config_state: ConfigState::new(),
                binds_state: BindsState::new(),
                settings: Settings::default(),
            },
            Task::done(GlobalMessagens::InitConfig),
        )
    }

    pub fn theme(&self) -> Theme {
        if let Some(theme) = self.ui_state.current_theme.to_owned() {
            return theme;
        }
        Theme::Dark
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tab {
    pub tab_name: String,
    pub path: PathBuf,
}

pub struct UiState {
    pub current_page: Page,
    pub current_theme: Option<Theme>,
    pub editor_grid: pane_grid::State<Pane>,
    pub editor_pane: pane_grid::Pane,
    pub file_tree_pane: pane_grid::Pane,
    pub file_tree_resize: (pane_grid::Split, f32),
    pub file_tree_is_open: bool,
    pub terminal_pane: pane_grid::Pane,
    pub terminal_pane_is_open: bool,
    pub terminal: iced_term::Terminal,
    pub editor: CodeEditor,
    pub tree: DirectoryTree,
    pub tabs: HashSet<Tab>,
}

impl UiState {
    pub fn new() -> Self {
        let (mut state, file_tree_pane) = pane_grid::State::new(Pane::FileTree);

        let (editor_pane, split) = state
            .split(pane_grid::Axis::Vertical, file_tree_pane, Pane::Editor)
            .unwrap();

        let (terminal_pane, _) = state
            .split(pane_grid::Axis::Horizontal, editor_pane, Pane::Terminal)
            .unwrap();

        state.close(terminal_pane);

        let tree = DirectoryTree::new(PathBuf::new())
            .with_filter(iced_swdir_tree::DirectoryFilter::FilesAndFolders);

        let mut editor = CodeEditor::new("", "rs").with_wrap_enabled(false);
        editor.set_theme(iced_code_editor::from_iced_theme(&Theme::CatppuccinMocha));

        let system_shell = std::env::var("SHELL")
            .expect("SHELL variable is not defined")
            .to_string();
        let term_settings = iced_term::settings::Settings {
            backend: iced_term::settings::BackendSettings {
                program: system_shell,
                ..Default::default()
            },
            ..Default::default()
        };

        let terminal = iced_term::Terminal::new(0, term_settings)
            .expect("failed to create the new terminal instance");

        Self {
            current_page: Page::EditorPage,
            current_theme: Some(Theme::CatppuccinMocha),
            editor_grid: state,
            file_tree_pane: file_tree_pane,
            file_tree_resize: (split, 0.),
            editor_pane: editor_pane,
            file_tree_is_open: true,
            terminal_pane_is_open: false,
            terminal_pane: terminal_pane,
            terminal: terminal,
            editor,
            tree,
            tabs: HashSet::new(),
        }
    }
}

pub struct DirState {
    pub current_dir_path: Option<PathBuf>,
    pub current_file_path: Option<PathBuf>,
}

impl DirState {
    pub fn new() -> Self {
        Self {
            current_dir_path: None,
            current_file_path: None,
        }
    }
}

pub struct ConfigState {
    pub auto_save_is_active: bool,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            auto_save_is_active: false,
        }
    }
}

pub struct BindsState {
    pub open_file_tree_bind: (keyboard::Modifiers, keyboard::Key),
}

impl BindsState {
    pub fn new() -> Self {
        Self {
            open_file_tree_bind: (
                keyboard::Modifiers::CTRL,
                keyboard::Key::Character("b".into()),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppTheme {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl From<AppTheme> for Theme {
    fn from(theme: AppTheme) -> Self {
        match theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
            AppTheme::Dracula => Theme::Dracula,
            AppTheme::Nord => Theme::Nord,
            AppTheme::SolarizedLight => Theme::SolarizedLight,
            AppTheme::SolarizedDark => Theme::SolarizedDark,
            AppTheme::GruvboxLight => Theme::GruvboxLight,
            AppTheme::GruvboxDark => Theme::GruvboxDark,
            AppTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            AppTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            AppTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            AppTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            AppTheme::TokyoNight => Theme::TokyoNight,
            AppTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            AppTheme::TokyoNightLight => Theme::TokyoNightLight,
            AppTheme::KanagawaWave => Theme::KanagawaWave,
            AppTheme::KanagawaDragon => Theme::KanagawaDragon,
            AppTheme::KanagawaLotus => Theme::KanagawaLotus,
            AppTheme::Moonfly => Theme::Moonfly,
            AppTheme::Nightfly => Theme::Nightfly,
            AppTheme::Oxocarbon => Theme::Oxocarbon,
            AppTheme::Ferra => Theme::Ferra,
        }
    }
}
