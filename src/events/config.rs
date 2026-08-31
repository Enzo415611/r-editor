use iced::{Task, Theme};

use crate::{
    state::{AppTheme, GlobalState},
    ui::config_page::ConfigSelected,
    update::GlobalMessagens,
};

impl GlobalState {
    pub fn config_update(&mut self, e: ConfigSelected) -> Task<GlobalMessagens> {
        match e {
            ConfigSelected::Theme(t) => {
                self.settings.current_theme = t.clone().into();
                if let Err(err) = self.save_settings() {
                    eprintln!("{}", err)
                }
                self.ui_state
                    .editor
                    .set_theme(iced_code_editor::from_iced_theme(&t));
                self.ui_state.current_theme = Some(t);

                Task::none()
            }
        }
    }
}
// set_font_size
// set_line_height
// set_line_numbers_enabled
// set_wrap_enabled

impl From<Theme> for AppTheme {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => AppTheme::Light,
            Theme::Dark => AppTheme::Dark,
            Theme::Dracula => AppTheme::Dracula,
            Theme::Nord => AppTheme::Nord,
            Theme::SolarizedLight => AppTheme::SolarizedLight,
            Theme::SolarizedDark => AppTheme::SolarizedDark,
            Theme::GruvboxLight => AppTheme::GruvboxLight,
            Theme::GruvboxDark => AppTheme::GruvboxDark,
            Theme::CatppuccinLatte => AppTheme::CatppuccinLatte,
            Theme::CatppuccinFrappe => AppTheme::CatppuccinFrappe,
            Theme::CatppuccinMacchiato => AppTheme::CatppuccinMacchiato,
            Theme::CatppuccinMocha => AppTheme::CatppuccinMocha,
            Theme::TokyoNight => AppTheme::TokyoNight,
            Theme::TokyoNightStorm => AppTheme::TokyoNightStorm,
            Theme::TokyoNightLight => AppTheme::TokyoNightLight,
            Theme::KanagawaWave => AppTheme::KanagawaWave,
            Theme::KanagawaDragon => AppTheme::KanagawaDragon,
            Theme::KanagawaLotus => AppTheme::KanagawaLotus,
            Theme::Moonfly => AppTheme::Moonfly,
            Theme::Nightfly => AppTheme::Nightfly,
            Theme::Oxocarbon => AppTheme::Oxocarbon,
            Theme::Ferra => AppTheme::Ferra,
            Theme::Custom(_) => AppTheme::Dark,
        }
    }
}
