extern crate log;
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
            heartbeat_freq: configurable_conf
                .remove("heartbeat")
                .map(|beat| {
                    beat.into_int()
                        .expect("Heartbeat Frequentency should in num seconds")
                })
                .unwrap_or(300),
            poll_agent_freq: configurable_conf
                .remove("poll")
                .map(|poll| poll.into_int().expect("polling should in num seconds"))
                .unwrap_or(600),
            ignore_host: {
                let mut hosts = Vec::new();
                if let Some(parsed_hosts) = configurable_conf.remove("ignore_host") {
                    for host in parsed_hosts.into_array().unwrap() {
                        hosts.push(host.into_string().unwrap())
                    }
                } else {
                    log::debug!("There is no hosts to ignore")
                }
                Some(HostIp { host_ip: hosts })
            },
            controller_port: configurable_conf
                .remove("controller_port")
                .map(|port| port.into_int().expect("Should be a port"))
                .unwrap_or(19200),
            authentication: configurable_conf
                .remove("authentication")
                .map(|auth| auth.into_bool().expect("Option is not True/False"))
                .unwrap_or(true),
            db_server: {
                let mut hosts = Vec::new();
                if let Some(parsed_hosts) = configurable_conf.remove("db_server") {
                    for host in parsed_hosts.into_array().unwrap() {
                        hosts.push(host.into_string().unwrap())
                    }
                } else {
                    log::info!("There is no hosts to process")
                }
                Some(HostIp { host_ip: hosts })
            },
            cassandra_options: if let Some(cass) = configurable_conf.remove("cassandra") {
                let tab = cass.into_table().expect("Configuration File does not contain cassandra. See Documentation for Configuration");
                log::debug!("{:?}", &tab);
                Some(CassandraOptions::parse(tab))
            } else {
                log::info!("Failed");
                None
            },
        }
    }
}
impl Config for AgentConfigRules {
    fn parse(conf: config::Map<String, config::Value>) -> Self {
        let mut configurable_conf = conf;
        Self {
            query_system_freq: configurable_conf
                .remove("system_freq")
                .map(|freq| {
                    freq.into_int()
                        .expect("Query polling should in num seconds")
                })
                .unwrap_or(300),
            no_cache: configurable_conf
                .remove("no_cache")
                .map(|cache| cache.into_bool().expect("Option is not True/False"))
                .unwrap_or(false),
            auto_discovery: configurable_conf
                .remove("auto_discovery")
                .map(|disc| disc.into_bool().expect("Input is not True/False"))
                .unwrap_or(false),
            controller_ip: configurable_conf
                .remove("controller_ip")
                .map(|ip| ip.into_string().unwrap())
                .expect("ControllerIP is not set, Please read Documentation"),
            token: configurable_conf
                .remove("token")
                .map(|token| token.into_string().unwrap_or_else(|_| String::from("")))
                .unwrap(),
            port: configurable_conf
                .remove("port")
                .map(|port| port.into_int().expect("Not a Valid Port Number"))
                .unwrap_or(19200),
        }
    }
}

impl Config for CassandraOptions {
    fn parse(conf: config::Map<String, config::Value>) -> Self {
        let mut configurable_conf = conf;
        log::debug!("Cassandra Configuration: {:?}", &configurable_conf);
        log::info!("Loading Cassandra Options");
        Self {
            replication: if let Some(rep) = configurable_conf.remove("replication") {
                match rep.into_string().unwrap().as_str() {
                    "SimpleStrategy" => ReplicationStrategy::SimpleStrategy.into_int(),
                    "NetworkTopologyStrategy" => {
                        ReplicationStrategy::NetworkTopologyStrategy.into_int()
                    }
                    _ => ReplicationStrategy::SimpleStrategy.into_int(),
                }
            } else {
                ReplicationStrategy::SimpleStrategy.into_int()
            },
            passwd: configurable_conf
                .remove("password")
                .map(|passwd| -> String { ToString::to_string(&passwd) }),
            user: configurable_conf
                .remove("user")
                .map(|user| -> String { ToString::to_string(&user) }),
            replication_factor: configurable_conf
                .remove("replication_factor")
                .map(|factor| factor.into_int().unwrap()),
            datacenters_mapping: configurable_conf.remove("mapping").map(|_| DcMapping {
                mapping: HashMap::from([(String::from("Dc1"), 5)]),
            }),
        }
    }
}
// Build a new impl to make it easier to get replications
impl CassandraOptions {
    pub fn get_replication(&self) -> ReplicationStrategy {
        ReplicationStrategy::from_i32(self.replication).expect("Invalid Replication Strategy")
    }
}
impl ReplicationStrategy {
    pub fn into_int(&self) -> i32 {
        *self as i32
    }
}
