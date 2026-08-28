use crate::state::{GlobalState, Settings};

impl GlobalState {
    pub fn load_settings(&mut self) -> anyhow::Result<()> {
        let s = confy::load::<Settings>("r-editor", "settings")?;
        self.settings = s;
        Ok(())
    }

    pub fn save_settings(&mut self, new_setting: Settings) -> anyhow::Result<()> {
        self.settings = new_setting.clone();
        confy::store::<Settings>("r-editor", "settings", new_setting)?;

        Ok(())
    }
}
