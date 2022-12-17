extern crate log;
use crate::retrieval::retrieval_trait::traits;
use config::{Config, File, FileFormat};

#[derive(Debug)]
pub struct FileRetrieve {
    pub path: String,
}

impl std::ops::Deref for FileRetrieve {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
impl traits::Retrieval<Config> for FileRetrieve {
    fn retreieve<FileRetrieve>(self) -> Option<Config> {
        log::debug!("Using Path {} to build a config", &self.path);
        let settings = Config::builder()
            .add_source(File::new(self.path.as_str(), FileFormat::Yaml))
            .build();
        log::trace!("Settings ConfigBuild {:?}", &settings);
        Some(settings.unwrap_or_else(|error| panic!("Failed to get Config {}", error)))
    }
}
