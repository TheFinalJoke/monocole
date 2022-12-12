extern crate log;
pub mod cassandra;
pub mod logging;
pub mod retrieval;
pub mod rpc_types;
use crate::retrieval::retrieval_trait::traits::Retrieval;
use crate::retrieval::config_lib::FileRetrieve;
use crate::rpc_types::config_types::{self, Config};

pub fn get_controller_configuration(path: FileRetrieve) -> config_types::ControllerConfigRules{
    log::info!("Pulling Config From {}", &path.path);
    let configuration = path.retreieve::<FileRetrieve>().unwrap().get_table("controller").unwrap_or_else(|_| {
        panic!("Configuration File does not contain controller. See Documentation for Configuration")
    });
    config_types::ControllerConfigRules::parse(configuration)
}


pub fn get_agent_configuration(path: FileRetrieve) -> config_types::AgentConfigRules{
    log::info!("Pulling Config From {}", &path.path);
    let configuration = path.retreieve::<FileRetrieve>().unwrap().get_table("agent").unwrap_or_else(|_| {
        panic!("Configuration File does not contain Agent. See Documentation for Configuration")
    });
    config_types::AgentConfigRules::parse(configuration)
}

// Will have to figure out how to mock properly
// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//
//     use crate::{retrieval::retrieval_trait::traits::MockRetrieval};
//     use config as config_crate;
//     use config::{Value, ValueKind};
//     use super::*;
//     enum ConfigType {
//         AGENT=0,
//         CONTROLLER=1,
//     }
//     
//     fn gen_config(config_type: ConfigType) -> config_crate::Config {
//         let builder = config_crate::Config::builder();
//         match config_type {
//             ConfigType::AGENT => {
//                 builder.set_default("Agent", "Nothing").unwrap().build().unwrap()
//             },
//             ConfigType::CONTROLLER => {
//                 builder.set_default("controller", "Nothing").unwrap().build().unwrap()
//             }
//         }
//     }
//     #[test]
//     fn test_get_controller_configuration() {
//         let file_path = FileRetrieve {
//             path: String::from("test_path"),
//         };
//         println!("{:?}", gen_config(ConfigType::AGENT));
//         let mut mocked:MockRetrieval<FileRetrieve> = MockRetrieval::new();
//         mocked.expect_retreieve::<config_crate::Config>().return_const(gen_config(ConfigType::CONTROLLER));
//         // let conf = get_controller_configuration(file_path);
//         // dbg!(conf);
//     }
// }