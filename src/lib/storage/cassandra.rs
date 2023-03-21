// First Get the options that we need for cassandra
// Make sure we can get a connection with a password or Nah
// - TODO: Add Password and Username -> Add support for ENV
// Add Ability to have different Replications
// Network Topologies -> Allow Different Datacenters
// load_balancing
// Default Timeout
use crate::rpc_types::{config_types, hardware_types};
use scylla::batch::Batch;
use scylla::frame::response::result::Row;
use scylla::transport::errors::NewSessionError;
use scylla::{Session, SessionBuilder};
use std::rc::Rc;
use std::time::Duration;
use thiserror::Error;

pub enum QueryStatus {
    COMPLETED = 0,
    ADDED = 1,
    FAILED = 2,
    SUCCESS = 3,
}
pub async fn connect(
    config_rules: Rc<config_types::ControllerConfigRules>,
) -> Result<Session, NewSessionError> {
    let session = SessionBuilder::new()
        .known_nodes(&config_rules.db_server.as_ref().unwrap().host_ip)
        .connection_timeout(Duration::from_secs(5))
        .build()
        .await?;
    Ok(session)
}
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Connection was Interrupt")]
    ConnectionInterrupt(String),
    #[error("Invaild Query")]
    QueryInvalid(String),
    #[error("Could not establish connection")]
    ConnectionInvalid,
}
#[derive(Debug)]
pub struct Cql {
    pub keyspace: String,
    pub config_rules: Rc<config_types::ControllerConfigRules>,
    pub session: Session,
}
impl Cql {
    pub async fn keyspaces(&self) -> Option<Vec<Row>> {
        match self.session.query("DESC KEYSPACES;", &[]).await {
            Ok(res) => res.rows,
            Err(_) => None,
        }
    }
    pub async fn build_keyspace(&self) -> Result<(), NewSessionError> {
        let options = self.config_rules.as_ref().cassandra_options.as_ref();
        self.session.query(
            format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': '{}', 'replication_factor': '{}'}}", 
            self.keyspace.as_str(),
            options.unwrap().replication().as_str_name(), options.unwrap().replication_factor.unwrap_or(1)),
            &[]).await?;
        Ok(())
    }
    pub async fn drop_keyspace(&self) -> Result<(), NewSessionError> {
        self.session
            .query(
                format!("DROP KEYSPACE IF EXISTS {}", self.keyspace.as_str()),
                &[],
            )
            .await?;
        Ok(())
    }
    pub async fn develop_datatypes(&self) -> Result<(), NewSessionError> {
        self.session
            .use_keyspace(self.keyspace.as_str(), false)
            .await?;
        // TODO Build async or batch query
        for query in hardware_types::generate_datatypes() {
            self.session.query(query, ()).await?;
        }
        Ok(())
    }
    pub async fn create_table(&self, query: String) -> Result<(), NewSessionError> {
        self.session
            .use_keyspace(self.keyspace.as_str(), false)
            .await?;
        self.session.query(query, ()).await?;
        Ok(())
    }
    pub async fn initialize_table(&self) -> Result<(), NewSessionError> {
        let query = format!(
            "CREATE TABLE monocole( 
            host_id UUID,
            timestamp timeUUID,
            hostname text,
            ip_address text,
            is_active text,
            is_container boolean,
            is_baremetal boolean,
            is_virtual boolean,
            is_vm boolean,
            cpu cpus,
            dimms Dimms,
            nics Nics,
            disks Disks,
            mb Motherboard,
            PRIMARY KEY ((host_id, hostname, ip_address))
        )"
        );
        self.create_table(query).await?;
        Ok(())
    }
    pub async fn insert_single<T: scylla::frame::value::Value>(
        &self,
        table: &'static str,
        insert: T,
    ) -> Result<(), NewSessionError> {
        self.session
            .use_keyspace(self.keyspace.as_str(), false)
            .await?;

        self.session
            .query(format!("INSERT INTO {} (a) VALUES (?)", table), (insert,))
            .await?;
        Ok(())
    }
    pub async fn batch_insert(&self, batched_queries: Vec<&str>) -> Result<(), NewSessionError> {
        self.session
            .use_keyspace(self.keyspace.as_str(), false)
            .await?;
        let mut batch: Batch = Default::default();
        let mut new_batch_values: Vec<()> = vec![];
        for query in batched_queries {
            batch.append_statement(query);
            new_batch_values.push(())
        }
        self.session.batch(&batch, new_batch_values).await?;
        Ok(())
    }
}
