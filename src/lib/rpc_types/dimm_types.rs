pub fn generate_dimm_datatype() -> &'static str {
    r#"CREATE TYPE Dimm (
        vendor text,
        model text,
        mem_type int, 
        max_clock double,
        min_clock double,
        size int,
    );
    "#
}

pub fn generate_dimms_datatype() -> &'static str {
    r#"CREATE TYPE Dimms (
        dimms frozen<Dimm>,
    );
    "#
}
