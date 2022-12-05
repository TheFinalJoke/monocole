use crate::config_types::Config;
extern crate log;
pub mod logging;
pub mod retrieval;
use crate::retrieval::retrieval_trait::traits::Retrieval;
use crate::retrieval::config_lib::FileRetrieve;

pub mod config_types {
    #[cfg(test)]
    use mockall::automock;
    
    #[cfg_attr(test, automock)]
    pub trait Config {
        fn parse(conf: config::Map<String, config::Value>) -> Self;
    }
    tonic::include_proto!("config");

    impl Config for ControllerConfigRules { 
        fn parse(conf: config::Map<String, config::Value>) -> Self {
            let mut configurable_conf = conf;
            Self {
                heartbeat_freq: match configurable_conf.remove("heartbeat") {
                    Some(beat) => beat.into_int().unwrap(),
                    None => 300,
                },
                poll_agent_freq: match configurable_conf.remove("poll") {
                    Some(polled) => polled.into_int().unwrap(),
                    None => 600,
                },
                ignore_host: {
                    let mut hosts = Vec::new();
                        match configurable_conf.remove("ignore_host") {
                            Some(parsed_hosts) => {
                                for host in parsed_hosts.into_array().unwrap() {
                                    hosts.push(host.into_string().unwrap())
                                }
                            },
                            None => {log::info!("There is no hosts to process")},
                        }
                        Some(
                        HostIp{
                            host_ip: hosts
                        })
                },
                controller_port: match configurable_conf.remove("controller_port") {
                    Some(port) => port.into_int().unwrap(),
                    None => 19200,
                },
                db_port: match configurable_conf.remove("db_port") {
                    Some(port) => port.into_int().unwrap(),
                    None => 9042,
                },
                authentication: match configurable_conf.remove("authentication") {
                    Some(auth) => auth.into_bool().unwrap(),
                    None => true,
                },
            }
        }
    }
    impl Config for AgentConfigRules {

        fn parse(conf: config::Map<String, config::Value>) -> Self {
            let mut configurable_conf = conf;
            Self {
                query_system_freq: match configurable_conf.remove("system_freq") {
                    Some(freq) => freq.into_int().unwrap(),
                    None => 300,
                },
                no_cache: match configurable_conf.remove("no_cache") {
                    Some(cache) => cache.into_bool().unwrap(),
                    None => false
                },
                auto_discovery: match configurable_conf.remove("auto_discovery") {
                    Some(disc) => disc.into_bool().unwrap(),
                    None => false
                },
                controller_ip: match configurable_conf.remove("controller_ip") {
                    Some(ip) => ip.into_string().unwrap(),
                    None => panic!("Controller IP is not set, Please read documentation")
                },
                token: match configurable_conf.remove("token") {
                    Some(token) => token.into_string().unwrap(),
                    None => "".to_string()
                },
                port: match configurable_conf.remove("port") {
                    Some(port) => port.into_int().unwrap(),
                    None => 19200
                }, 
            }
        }
    }
}

pub mod control_types {
    tonic::include_proto!("controller");
}
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