/// Site-local defaults
use async_trait::async_trait;
use user_data_provider::provider::{user_data_from_file, UserDataProvider};
use user_data_provider::settings::SettingsJson;

const LOCAL_DEFAULTS_FILE: &str = "/local/user-data-defaults.toml";

pub struct LocalDefaults;

#[async_trait]
impl UserDataProvider for LocalDefaults {
    async fn user_data(&self) -> Result<Option<SettingsJson>, Box<dyn std::error::Error>> {
        user_data_from_file(LOCAL_DEFAULTS_FILE)
    }
}
