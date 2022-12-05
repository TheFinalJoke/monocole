use cdrs_tokio::cluster::session::{TcpSessionBuilder, SessionBuilder};
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use log::{info};
pub async fn run() {
    
    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point("192.168.200.117:9042".into())
        .build()
        .await
        .unwrap();
    let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config)
        .build()
        .unwrap();
    let create_ks = "CREATE KEYSPACE IF NOT EXISTS test_ks WITH REPLICATION = { \
                     'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    info!("{}", create_ks);
    session.query(create_ks).await.expect("Keyspace create error");
}