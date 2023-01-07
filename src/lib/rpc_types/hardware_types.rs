use std::fmt::Debug;

tonic::include_proto!("hardware");
pub struct CreateDatatype{}

pub fn generate_datatypes() -> Vec<&'static str> {
    vec![
        // Generate CPU DataType
        "CREATE TYPE cpu {
            vendor text,
            arch text,
            num_of_cores int,
            model text,
            num_of_threads_per_core int,
            max_speed double,
            min_speed double,
            boost_clock boolean,
        };",
        // Generate Cpus DataType
        "CREATE TYPE Cpus {
            cpu_list list<cpu>  
        };",
    ]
}
