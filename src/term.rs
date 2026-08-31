use std::{collections::HashMap, env, path::PathBuf};

use iced_term::Terminal;

use crate::state::GlobalState;

impl GlobalState {
    pub fn new_terminal(&self, id: u64) -> Terminal {
        let path = self.dir_state.current_dir_path.clone();

        let term_settings = iced_term::settings::Settings {
            backend: iced_term::settings::BackendSettings {
                program: get_system_shell(),
                env: env_for_terminal(),
                working_directory: path,
                ..Default::default()
            },
            ..Default::default()
        };
        iced_term::Terminal::new(id, term_settings)
            .expect("failed to create the new terminal instance")
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
