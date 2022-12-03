extern crate log;
use config::{File, Config, FileFormat};
use std::path::Path;

pub fn check_for_config_file(path: &String) -> bool {
    Path::new(path).exists()  
}

pub fn get_settings_from_config_file(path: String) -> Config {
    log::debug!("Using Path {} to build a config", &path);
    let settings = Config::builder()
        .add_source(File::new(path.as_str(), FileFormat::Yaml))
        .build();
    log::trace!("Settings ConfigBuild {:?}", &settings);
    settings.unwrap_or_else(|error| {
            panic!("Failed to get Config {}", error)
    })
}