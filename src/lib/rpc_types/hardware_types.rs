use std::fmt::Debug;

tonic::include_proto!("hardware");
pub struct CreateDatatype{}

pub fn generate_datatypes() -> Vec<&'static str> {
    vec![
        // Generate CPU DataType
        "CREATE TYPE Cpu (
            vendor text,
            arch text,
            num_of_cores int,
            model text,
            num_of_threads_per_core int,
            max_speed double,
            min_speed double,
            boost_clock boolean,
        );",
        // Generate Cpus DataType
        "CREATE TYPE Cpus (
            cpu_list frozen<cpu>  
        );",
        // Generate Motherboard Datatypes
        "CREATE TYPE Motherboard(
            vendor text,
            model text,
            socket_type text,
            bios text,
        );",
        "CREATE TYPE Dimm (
            vendor text,
            model text,
            mem_type int, 
            max_clock double,
            min_clock double,
            size int,
        );
        ",
        "CREATE TYPE Dimms (
            dimms frozen<Dimm>,
        );
        ",
        "CREATE TYPE Drive (
            vendor text,
            model text,
            type int,
            size int,
            spin_speed int,
            transfer_speed int,
            pci_address text,
            logical_location text,
        );
        ",
        "CREATE TYPE Disks(
            disks frozen<Drive>
        );
        ",
        "CREATE TYPE NetworkInterfaceCard (
            vendor text,
            model text,
            interface_name text,
            mac_address text,
            autonegotiation bool,
            driver text,
            speed text,
            duplex int,
            ip_addr text,
            iptype int,
            physical_location text,
        );
        ",
    ]
}
