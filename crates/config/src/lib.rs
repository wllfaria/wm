mod config;
mod config_loader;
mod keysyms;

pub use config::Config;
use config_loader::UnresolvedConfig;
use std::path::{Path, PathBuf};

static APP_NAME: &str = "lucky";
static CONFIG_FILE: &str = "config.toml";
static XDG_HOME: &str = "XDG_CONFIG_HOME";
static LUCKY_CONF_ENV_VAR: &str = "LUCKY_CONFIG";

/// Verify if `XDG_HOME`/.config/lucky/config.toml exists and returns the path to it in case it
/// exists
fn get_config_dir_path() -> Option<PathBuf> {
    let var = match std::env::var(XDG_HOME) {
        Ok(home_path) => Some(
            Path::new(&home_path)
                .join(".config")
                .join(APP_NAME)
                .join(CONFIG_FILE),
        ),
        Err(_) => None,
    };
    var
}

/// Try to load the config from the file on a given path
fn load_config_from_file<P>(path: P) -> anyhow::Result<Config>
where
    P: AsRef<Path>,
{
    let config_file = std::fs::read_to_string(path.as_ref())?;
    let config = toml::from_str::<UnresolvedConfig>(&config_file)?;
    match Config::try_from(config) {
        Ok(config) => Ok(config),
        Err(_) => anyhow::bail!("failed to parse config file"),
    }
}

/// Try to load the configuration from 3 places, in the following order:
///
/// * If set, `LUCKY_CONFIG` will be prioritized and the config will be loaded from there;
/// * If not, will attempt to load from `XDG_HOME`/.config/lucky/config.toml;
/// * If not present on any of the directories above, will load the default configuration;
pub fn load_config() -> anyhow::Result<Config> {
    let config_path = match std::env::var(LUCKY_CONF_ENV_VAR) {
        Ok(var) => Some(PathBuf::from(&var).join(CONFIG_FILE)),
        Err(_) => get_config_dir_path(),
    };
    config_path
        .map(load_config_from_file)
        .unwrap_or(Ok(Default::default()))
}