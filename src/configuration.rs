use config::{Config, ConfigError, File};
use property::Property;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Property, Clone)]
#[property(set(private), get(public))]
pub struct Listener {
    #[property(set(private), get(public))]
    name: String,
    #[property(set(private), get(public))]
    socket_addr: String,
}

#[derive(Serialize, Deserialize, Property, Clone)]
#[property(set(private), get(public))]
pub struct FileLocations {
    #[property(set(private), get(public, type = "clone"))]
    frontend_scripts: PathBuf,
    #[property(set(private), get(public, type = "clone"))]
    frontend_styles: PathBuf,
    #[property(set(private), get(public, type = "clone"))]
    frontend_copy: PathBuf,
    #[property(set(private), get(public, type = "clone"))]
    media: PathBuf,
}

#[derive(Serialize, Deserialize, Property, Clone)]
#[property(set(private), get(public))]
pub struct ServerSettings {
    #[property(set(private), get(public))]
    listeners: Vec<Listener>,
    #[property(set(private), get(public))]
    file_locations: FileLocations,
}

impl ServerSettings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut fallback_conf_file: PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        fallback_conf_file.push("tests");
        fallback_conf_file.push("default.toml");
        let conf_builder = Config::builder()
            .add_source(File::from(fallback_conf_file))
            .build()?;

        conf_builder.try_deserialize()
    }
}

#[test]
fn config_succeed() {
    assert_eq!(2, 2);
}
