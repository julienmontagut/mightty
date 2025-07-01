use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use std::{env, fs, io};

use figment::{
    Error as FigmentError, Figment, Metadata, Profile, Provider,
    providers::{Env, Format, Serialized, Toml},
    value::{Dict, Tag, Value as FigmentValue},
};
use log::{debug, error, info};
use serde::Deserialize;
use std::collections::BTreeMap;
use toml::de::Error as TomlError;
use toml::ser::Error as TomlSeError;
use toml::{Table, Value};

pub mod bell;
pub mod color;
pub mod cursor;
pub mod debug;
pub mod defaults;
pub mod font;
pub mod general;
pub mod monitor;
pub mod scrolling;
pub mod selection;
pub mod serde_utils;
pub mod terminal;
pub mod ui_config;
pub mod window;

mod bindings;
mod mouse;

use crate::cli::Options;
#[cfg(test)]
pub use crate::config::bindings::Binding;
pub use crate::config::bindings::{
    Action, BindingKey, BindingMode, KeyBinding, MouseAction, SearchAction, ViAction,
};
pub use crate::config::ui_config::UiConfig;
use crate::logging::LOG_TARGET_CONFIG;

/// Maximum number of depth for the configuration file imports.
pub const IMPORT_RECURSION_LIMIT: usize = 5;

/// Result from config loading.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors occurring during config loading.
#[derive(Debug)]
pub enum Error {
    /// Couldn't read $HOME environment variable.
    ReadingEnvHome(env::VarError),

    /// io error reading file.
    Io(io::Error),

    /// Invalid toml.
    Toml(TomlError),

    /// Failed toml serialization.
    TomlSe(TomlSeError),

    /// Figment configuration error.
    Figment(figment::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReadingEnvHome(err) => err.source(),
            Error::Io(err) => err.source(),
            Error::Toml(err) => err.source(),
            Error::TomlSe(err) => err.source(),
            Error::Figment(err) => err.source(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadingEnvHome(err) => {
                write!(f, "Unable to read $HOME environment variable: {err}")
            }
            Error::Io(err) => write!(f, "Error reading config file: {err}"),
            Error::Toml(err) => write!(f, "Config error: {err}"),
            Error::TomlSe(err) => write!(f, "TOML serialization error: {err}"),
            Error::Figment(err) => write!(f, "Configuration error: {err}"),
        }
    }
}

impl From<env::VarError> for Error {
    fn from(val: env::VarError) -> Self {
        Error::ReadingEnvHome(val)
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::Io(val)
    }
}

impl From<TomlError> for Error {
    fn from(val: TomlError) -> Self {
        Error::Toml(val)
    }
}

impl From<TomlSeError> for Error {
    fn from(val: TomlSeError) -> Self {
        Error::TomlSe(val)
    }
}

impl From<figment::Error> for Error {
    fn from(val: figment::Error) -> Self {
        Error::Figment(val)
    }
}

/// Load the configuration file using Figment.
pub fn load(options: &mut Options) -> UiConfig {
    let config_path = options
        .config_file
        .clone()
        .or_else(|| installed_config("toml"));

    // Build Figment configuration provider
    let mut figment = Figment::new();

    figment = figment.merge(Env::prefixed("MIGHTTY_"));

    if let Some(ref path) = config_path {
        if path.exists() {
            figment = figment.merge(Toml::file(path));
        } else {
            info!(target: LOG_TARGET_CONFIG, "Config file not found: {:?}", path);
        }
    } else {
        info!(target: LOG_TARGET_CONFIG, "No config file found; using default");
    }

    figment = figment.merge(CliProvider::from_options(options));

    // Load configuration with imports
    let mut config = load_with_figment(figment, config_path.as_deref()).unwrap_or_else(|err| {
        error!(target: LOG_TARGET_CONFIG, "Failed to load config: {}", err);
        let mut config = UiConfig::default();
        if let Some(path) = config_path {
            config.config_paths.push(path);
        }
        config
    });

    // Apply any remaining CLI-specific logic not handled by Figment
    after_loading(&mut config, options);

    config
}

/// Load configuration with Figment, handling imports
fn load_with_figment(figment: Figment, config_path: Option<&Path>) -> Result<UiConfig> {
    let mut config_paths = Vec::new();

    let config_value: Value = figment.extract()?;
    let final_config = if let Some(path) = config_path {
        config_paths.push(path.to_owned());
        load_config_with_imports(
            config_value,
            path,
            &mut config_paths,
            IMPORT_RECURSION_LIMIT,
        )?
    } else {
        config_value
    };

    let mut config = UiConfig::deserialize(final_config)?;
    config.config_paths = config_paths;

    Ok(config)
}

/// Load configuration handling imports recursively
fn load_config_with_imports(
    mut config: Value,
    base_path: &Path,
    config_paths: &mut Vec<PathBuf>,
    recursion_limit: usize,
) -> Result<Value> {
    // Load imports and merge them
    let imports = load_imports(&config, base_path, config_paths, recursion_limit);
    config = serde_utils::merge(imports, config);
    Ok(config)
}

/// Attempt to reload the configuration file.
pub fn reload(config_path: &Path, options: &mut Options) -> Result<UiConfig> {
    debug!("Reloading configuration file: {:?}", config_path);

    // Build Figment configuration provider for reload
    let figment = Figment::new()
        .merge(Env::prefixed("MIGHTTY_"))
        .merge(Toml::file(config_path))
        .merge(CliProvider::from_options(options));

    // Load config, propagating errors.
    let mut config = load_with_figment(figment, Some(config_path))?;

    after_loading(&mut config, options);

    Ok(config)
}

/// Modifications after the `UiConfig` object is created.
fn after_loading(config: &mut UiConfig, _options: &mut Options) {
    // Most CLI overrides are now handled by Figment.
    // Only special CLI logic that can't be handled by Figment providers goes here.

    // Force minimum log level for print_events
    if config.debug.print_events {
        config.debug.log_level = config.debug.log_level.max(log::LevelFilter::Info);
    }
}

fn parse_config(
    path: &Path,
    config_paths: &mut Vec<PathBuf>,
    recursion_limit: usize,
) -> Result<Value> {
    config_paths.push(path.to_owned());

    let config = deserialize_config(path)?;

    // Merge config with imports.
    let imports = load_imports(&config, path, config_paths, recursion_limit);
    Ok(serde_utils::merge(imports, config))
}

/// Load all referenced configuration files.
fn load_imports(
    config: &Value,
    base_path: &Path,
    config_paths: &mut Vec<PathBuf>,
    recursion_limit: usize,
) -> Value {
    let import_paths = match imports(config, base_path, recursion_limit) {
        Ok(import_paths) => import_paths,
        Err(err) => {
            error!(target: LOG_TARGET_CONFIG, "{err}");
            return Value::Table(Table::new());
        }
    };

    let mut merged = Value::Table(Table::new());
    for import_path in import_paths {
        let path = match import_path {
            Ok(path) => path,
            Err(err) => {
                error!(target: LOG_TARGET_CONFIG, "{err}");
                continue;
            }
        };

        match parse_config(&path, config_paths, recursion_limit - 1) {
            Ok(config) => merged = serde_utils::merge(merged, config),
            Err(Error::Io(io)) if io.kind() == io::ErrorKind::NotFound => {
                info!(target: LOG_TARGET_CONFIG, "Config import not found:\n  {:?}", path.display());
                continue;
            }
            Err(err) => {
                error!(target: LOG_TARGET_CONFIG, "Unable to import config {:?}: {}", path, err)
            }
        }
    }

    merged
}

/// Deserialize a configuration file.
pub fn deserialize_config(path: &Path) -> Result<Value> {
    let mut contents = fs::read_to_string(path)?;

    if contents.starts_with('\u{FEFF}') {
        contents = contents.split_off(3);
    }

    let config: Value = toml::from_str(&contents)?;

    Ok(config)
}

/// Get all import paths for a configuration.
pub fn imports(
    config: &Value,
    base_path: &Path,
    recursion_limit: usize,
) -> StdResult<Vec<StdResult<PathBuf, String>>, String> {
    let imports = config
        .get("import")
        .or_else(|| config.get("general").and_then(|g| g.get("import")));
    let imports = match imports {
        Some(Value::Array(imports)) => imports,
        Some(_) => return Err("Invalid import type: expected a sequence".into()),
        None => return Ok(Vec::new()),
    };

    // Limit recursion to prevent infinite loops.
    if !imports.is_empty() && recursion_limit == 0 {
        return Err("Exceeded maximum configuration import depth".into());
    }

    let mut import_paths = Vec::new();

    for import in imports {
        let path = match import {
            Value::String(path) => PathBuf::from(path),
            _ => {
                import_paths.push(Err(
                    "Invalid import element type: expected path string".into()
                ));
                continue;
            }
        };

        let normalized = normalize_import(base_path, path);

        import_paths.push(Ok(normalized));
    }

    Ok(import_paths)
}

/// Normalize import paths.
pub fn normalize_import(base_config_path: &Path, import_path: impl Into<PathBuf>) -> PathBuf {
    let mut import_path = import_path.into();

    // Resolve paths relative to user's home directory.
    if let (Ok(stripped), Some(home_dir)) = (import_path.strip_prefix("~/"), home::home_dir()) {
        import_path = home_dir.join(stripped);
    }

    if import_path.is_relative() {
        if let Some(base_config_dir) = base_config_path.parent() {
            import_path = base_config_dir.join(import_path)
        }
    }

    import_path
}

/// Custom Figment provider for CLI arguments.
pub struct CliProvider {
    /// CLI arguments as a Figment-compatible value dictionary
    dict: Dict,
    /// Metadata for the provider
    metadata: Metadata,
}

impl CliProvider {
    /// Create a new CLI provider from parsed Options
    pub fn from_options(options: &Options) -> Self {
        let mut dict = Dict::new();

        // Add direct CLI field mappings
        if options.print_events {
            Self::set_nested_value(
                &mut dict,
                "debug.print_events",
                FigmentValue::Bool(Tag::Default, true),
            );
        }

        if options.ref_test {
            Self::set_nested_value(
                &mut dict,
                "debug.ref_test",
                FigmentValue::Bool(Tag::Default, true),
            );
        }

        if options.socket.is_some() {
            Self::set_nested_value(
                &mut dict,
                "general.ipc_socket",
                FigmentValue::Bool(Tag::Default, true),
            );
        }

        // Add log level (computed from verbose/quiet flags)
        let log_level = options.log_level();
        let log_level_str = match log_level {
            log::LevelFilter::Off => "Off",
            log::LevelFilter::Error => "Error",
            log::LevelFilter::Warn => "Warn",
            log::LevelFilter::Info => "Info",
            log::LevelFilter::Debug => "Debug",
            log::LevelFilter::Trace => "Trace",
        };
        Self::set_nested_value(
            &mut dict,
            "debug.log_level",
            FigmentValue::String(Tag::Default, log_level_str.to_string()),
        );

        Self {
            dict,
            metadata: Metadata::named("CLI Arguments"),
        }
    }

    /// Helper to merge a TOML table into the dictionary
    fn merge_table_into_dict(dict: &mut Dict, table: toml::Table) {
        for (key, value) in table {
            let figment_value = Self::toml_value_to_figment_value(value);
            dict.insert(key, figment_value);
        }
    }

    /// Convert TOML value to Figment value
    fn toml_value_to_figment_value(value: toml::Value) -> FigmentValue {
        match value {
            toml::Value::String(s) => FigmentValue::String(Tag::Default, s),
            toml::Value::Integer(i) => FigmentValue::Num(Tag::Default, figment::value::Num::I64(i)),
            toml::Value::Float(f) => FigmentValue::Num(Tag::Default, figment::value::Num::F64(f)),
            toml::Value::Boolean(b) => FigmentValue::Bool(Tag::Default, b),
            toml::Value::Array(arr) => {
                let figment_arr: Vec<FigmentValue> = arr
                    .into_iter()
                    .map(Self::toml_value_to_figment_value)
                    .collect();
                FigmentValue::Array(Tag::Default, figment_arr)
            }
            toml::Value::Table(table) => {
                let mut figment_dict = Dict::new();
                for (k, v) in table {
                    figment_dict.insert(k, Self::toml_value_to_figment_value(v));
                }
                FigmentValue::Dict(Tag::Default, figment_dict)
            }
            toml::Value::Datetime(dt) => FigmentValue::String(Tag::Default, dt.to_string()),
        }
    }

    /// Helper to set nested values in the dictionary (e.g., "debug.print_events")
    fn set_nested_value(dict: &mut Dict, path: &str, value: FigmentValue) {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() == 1 {
            dict.insert(parts[0].to_string(), value);
            return;
        }

        let mut current = dict;
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part - insert the value
                current.insert(part.to_string(), value);
                break;
            } else {
                // Intermediate part - ensure there's a Dict
                let entry = current
                    .entry(part.to_string())
                    .or_insert_with(|| FigmentValue::Dict(Tag::Default, Dict::new()));
                if let FigmentValue::Dict(_, dict) = entry {
                    current = dict;
                } else {
                    // If it's not a dict, we can't navigate further
                    return;
                }
            }
        }
    }
}

impl Provider for CliProvider {
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn data(
        &self,
    ) -> std::result::Result<BTreeMap<Profile, BTreeMap<String, FigmentValue>>, FigmentError> {
        let mut map = BTreeMap::new();
        map.insert(Profile::Default, self.dict.clone());
        Ok(map)
    }
}

/// IPC configuration provider for runtime configuration updates.
pub struct IpcProvider {
    /// IPC configuration options as a Figment-compatible value dictionary
    dict: Dict,
    /// Metadata for the provider
    metadata: Metadata,
}

impl IpcProvider {
    /// Create a new IPC provider from IPC options
    pub fn from_options(options: &[String]) -> Self {
        let mut dict = Dict::new();

        for option in options {
            if let Ok(parsed_value) = toml::from_str::<toml::Value>(option) {
                if let toml::Value::Table(table) = parsed_value {
                    Self::merge_table_into_dict(&mut dict, table);
                }
            }
        }

        Self {
            dict,
            metadata: Metadata::named("IPC Configuration"),
        }
    }

    /// Helper to merge a TOML table into the dictionary
    fn merge_table_into_dict(dict: &mut Dict, table: toml::Table) {
        for (key, value) in table {
            let figment_value = Self::toml_value_to_figment_value(value);
            dict.insert(key, figment_value);
        }
    }

    /// Convert TOML value to Figment value
    fn toml_value_to_figment_value(value: toml::Value) -> FigmentValue {
        match value {
            toml::Value::String(s) => FigmentValue::String(Tag::Default, s),
            toml::Value::Integer(i) => FigmentValue::Num(Tag::Default, figment::value::Num::I64(i)),
            toml::Value::Float(f) => FigmentValue::Num(Tag::Default, figment::value::Num::F64(f)),
            toml::Value::Boolean(b) => FigmentValue::Bool(Tag::Default, b),
            toml::Value::Array(arr) => {
                let figment_arr: Vec<FigmentValue> = arr
                    .into_iter()
                    .map(Self::toml_value_to_figment_value)
                    .collect();
                FigmentValue::Array(Tag::Default, figment_arr)
            }
            toml::Value::Table(table) => {
                let mut figment_dict = Dict::new();
                for (k, v) in table {
                    figment_dict.insert(k, Self::toml_value_to_figment_value(v));
                }
                FigmentValue::Dict(Tag::Default, figment_dict)
            }
            toml::Value::Datetime(dt) => FigmentValue::String(Tag::Default, dt.to_string()),
        }
    }

    /// Apply IPC configuration to a config, returning a new config
    pub fn apply_to_config(
        &self,
        base_config: std::rc::Rc<UiConfig>,
    ) -> std::result::Result<std::rc::Rc<UiConfig>, Error> {
        if self.dict.is_empty() {
            return Ok(base_config);
        }

        let figment = Figment::new()
            .merge(Serialized::from((*base_config).clone(), Profile::Default))
            .merge(self.clone());

        let new_config: UiConfig = figment.extract().map_err(Error::Figment)?;
        Ok(std::rc::Rc::new(new_config))
    }

    /// Check if this provider has any configuration
    pub fn is_empty(&self) -> bool {
        self.dict.is_empty()
    }

    /// Combine this provider with another IPC provider
    pub fn merge_with(&mut self, other: &IpcProvider) {
        for (key, value) in &other.dict {
            self.dict.insert(key.clone(), value.clone());
        }
    }

    /// Clear all configuration
    pub fn clear(&mut self) {
        self.dict.clear();
    }
}

impl Clone for IpcProvider {
    fn clone(&self) -> Self {
        Self {
            dict: self.dict.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl Provider for IpcProvider {
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn data(
        &self,
    ) -> std::result::Result<BTreeMap<Profile, BTreeMap<String, FigmentValue>>, FigmentError> {
        let mut map = BTreeMap::new();
        map.insert(Profile::Default, self.dict.clone());
        Ok(map)
    }
}

/// Get the location of the first found default config file paths
/// according to the following order:
///
/// 1. $XDG_CONFIG_HOME/mightty/mightty.toml
/// 2. $XDG_CONFIG_HOME/mightty.toml
/// 3. $HOME/.config/mightty/mightty.toml
/// 4. $HOME/.mightty.toml
pub fn installed_config(suffix: &str) -> Option<PathBuf> {
    let file_name = format!("mightty.{suffix}");

    // Try using XDG location by default.
    xdg::BaseDirectories::with_prefix("mightty")
        .find_config_file(&file_name)
        .or_else(|| xdg::BaseDirectories::new().find_config_file(&file_name))
        .or_else(|| {
            if let Ok(home) = env::var("HOME") {
                // Fallback path: $HOME/.config/mightty/mightty.toml.
                let fallback = PathBuf::from(&home)
                    .join(".config/mightty")
                    .join(&file_name);
                if fallback.exists() {
                    return Some(fallback);
                }
                // Fallback path: $HOME/.mightty.toml.
                let hidden_name = format!(".{file_name}");
                let fallback = PathBuf::from(&home).join(hidden_name);
                if fallback.exists() {
                    return Some(fallback);
                }
            }
            None
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config() {
        toml::from_str::<UiConfig>("").unwrap();
    }
}
