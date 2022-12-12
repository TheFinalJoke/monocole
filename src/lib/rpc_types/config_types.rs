use std::collections::HashMap;

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
            authentication: match configurable_conf.remove("authentication") {
                Some(auth) => auth.into_bool().unwrap(),
                None => true,
            },
            db_server: {
                let mut hosts = Vec::new();
                    match configurable_conf.remove("db_server") {
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
            cassandra_options: if let Some(cass) = configurable_conf.remove("cassandra") {
                let tab = cass.into_table().unwrap_or_else(|_| {
                    panic!("Configuration File does not contain cassandra. See Documentation for Configuration")
                });
                dbg!(&tab);
                Some(CassandraOptions::parse(tab))
            } else {
                log::info!("Failed");
                None
            }
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

impl Config for CassandraOptions {
    fn parse(conf:config::Map<String,config::Value>) -> Self {
        let mut configurable_conf = conf;
        Self {
            replication: if let Some(rep) = configurable_conf.remove("replication") {
                match rep.into_string().unwrap().as_str() {
                    "SimpleStrategy" => ReplicationStrategy::SimpleStrategy.into_int(),
                    "NetworkTopologyStrategy" => ReplicationStrategy::NetworkTopologyStrategy.into_int(),
                    _ => ReplicationStrategy::SimpleStrategy.into_int()
                }
            } else {
                ReplicationStrategy::SimpleStrategy.into_int()
            },
            passwd: configurable_conf.remove("password").map(|passwd| -> String {ToString::to_string(&passwd)}),
            user: configurable_conf.remove("user").map(|user| -> String {ToString::to_string(&user)}),
            replication_factor: configurable_conf.remove("replication_factor").map(|factor| factor.into_int().unwrap()),
            datacenters_mapping: configurable_conf.remove("mapping").map(|_| DcMapping {
                    mapping: HashMap::from([(String::from("Dc1"), 5)])
                })
                    
        }
    }
}

impl ReplicationStrategy {
    pub fn into_int(&self) -> i32 {
        *self as i32
    }
}
