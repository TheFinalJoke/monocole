use crate::config_types::Config;
extern crate log;
pub mod cassandra;
pub mod logging;
pub mod config_lib;

pub mod config_types {
    pub trait Config {
        fn parse(conf: config::Map<String, config::Value>) -> Self;
        fn default() -> Self;

    }
    tonic::include_proto!("config");

    impl Config for ControllerConfigRules {
        fn default() -> Self {
            Self { 
                heartbeat_freq: 2,
                poll_agent_freq: 2, 
                ignore_host: None, 
                controller_port: 19200, 
                db_port: 1922, 
                authentication: false }
        }

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
        fn default() -> Self {
            Self {
                query_system_freq: 1,
                no_cache: true,
                auto_discovery: true,
                controller_ip: "cool".to_owned(),
                token: "123jkf;dajkf;dnafd;hafkdl;aj".to_owned(),
                port: 12239,
            }
        }

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

pub fn get_controller_configuration(path: String) -> config_types::ControllerConfigRules{
    log::info!("Pulling Config From {}", &path);
    let configuration = config_lib::get_settings_from_config_file(path).get_table("controller").unwrap_or_else(|_| {
        panic!("Configuration File does not contain controller. See Documentation for Configuration")
    });
    config_types::ControllerConfigRules::parse(configuration)
}


pub fn get_agent_configuration(path: String) -> config_types::AgentConfigRules{
    log::info!("Pulling Config From {}", &path);
    let configuration = config_lib::get_settings_from_config_file(path).get_table("agent").unwrap_or_else(|_| {
        panic!("Configuration File does not contain Agent. See Documentation for Configuration")
    });
    config_types::AgentConfigRules::parse(configuration)
}