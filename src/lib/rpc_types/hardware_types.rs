use crate::rpc_types::cpu_types;
use crate::rpc_types::dimm_types;
use crate::rpc_types::disk_types;
use crate::rpc_types::motherboard_types;
use crate::rpc_types::nic_types;
use std::fmt::Debug;
tonic::include_proto!("hardware");
pub fn generate_datatypes() -> Vec<&'static str> {
    vec![
        cpu_types::generate_cpu_datatype(),
        cpu_types::generate_cpus_datatype(),
        motherboard_types::generate_motherboard_datatype(),
        dimm_types::generate_dimm_datatype(),
        dimm_types::generate_dimms_datatype(),
        disk_types::generate_disk_datatype(),
        disk_types::generate_disks_datatype(),
        nic_types::generate_nic_datatype(),
        nic_types::generate_nics_datatype(),
    ]
}
