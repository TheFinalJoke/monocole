pub fn generate_nic_datatype() -> &'static str {
    r#"CREATE TYPE NetworkInterfaceCard (
        vendor text,
        model text,
        interface_name text,
        mac_address text,
        autonegotiation boolean,
        driver text,
        speed text,
        duplex int,
        ip_addr text,
        iptype int,
        physical_location text,
    );
    "#
}

pub fn generate_nics_datatype() -> &'static str {
    r#"CREATE TYPE Nics (
       nics_list frozen<NetworkInterfaceCard>
    );
    "#
}
