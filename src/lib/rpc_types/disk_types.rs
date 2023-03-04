pub fn generate_disk_datatype() -> &'static str {
    r#"CREATE TYPE Drive (
        vendor text,
        model text,
        type int,
        size int,
        spin_speed int,
        transfer_speed int,
        pci_address text,
        logical_location text,
    );
    "#
}

pub fn generate_disks_datatype() -> &'static str {
    r#"CREATE TYPE Disks(
        disks frozen<Drive>
    );
    "#
}