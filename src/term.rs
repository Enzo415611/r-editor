use std::{collections::HashMap, env, path::PathBuf};

use iced_term::Terminal;

use crate::state::GlobalState;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TerminalInfo {
    pub id: u64,
    pub working_dir: Option<PathBuf>,
    pub shell: String,
}

impl TerminalInfo {
    pub fn new(id: u64, working_dir: Option<PathBuf>, shell: String) -> Self {
        Self {
            id,
            working_dir,
            shell,
        }
    }
}

impl GlobalState {
    pub fn new_terminal(&self, id: u64) -> (TerminalInfo, Terminal) {
        let path = self.dir_state.current_dir_path.clone();
        let shell = get_system_shell();
        let term_settings = iced_term::settings::Settings {
            backend: iced_term::settings::BackendSettings {
                program: shell.clone(),
                env: env_for_terminal(),
                working_directory: path.clone(),
                ..Default::default()
            },
            ..Default::default()
        };

        let term_key = TerminalInfo::new(id, path, shell);

        (
            term_key,
            iced_term::Terminal::new(id, term_settings)
                .expect("failed to create the new terminal instance"),
        )
    }
}

fn env_for_terminal() -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("TERM".to_string(), "xterm-256color".to_string());

    if let Ok(path) = std::env::var("PATH") {
        env.insert("PATH".to_string(), path);
    }
    if let Ok(home) = std::env::var("HOME") {
        env.insert("HOME".to_string(), home);
    }
    env
}

fn get_system_shell() -> String {
    if let Ok(shell) = env::var("SHELL") {
        if !shell.is_empty() {
            return shell;
        }
    }

    #[cfg(target_os = "windows")]
    {
        if which::which("pwsh").is_ok() {
            return "powershell".to_string();
        }
        return env::var("COMSPEC").unwrap_or_ekse(|_| "cmd.exe".to_string());
    }

    #[cfg(not(windows))]
    {
        "/bin/sh".to_string()
    }
}
