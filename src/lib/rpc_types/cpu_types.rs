pub fn generate_cpu_datatype() -> &'static str {
    r#"CREATE TYPE Cpu (
        vendor text,
        arch text,
        num_of_cores int,
        model text,
        num_of_threads_per_core int,
        max_speed double,
        min_speed double,
        boost_clock boolean,
    );"#
}

pub fn generate_cpus_datatype() -> &'static str {
    r#"CREATE TYPE Cpus (
        cpu_list frozen<cpu>  
    );"#
}
