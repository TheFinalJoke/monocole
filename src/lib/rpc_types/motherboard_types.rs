pub fn generate_motherboard_datatype() -> &'static str {
    r#"CREATE TYPE Motherboard(
        vendor text,
        model text,
        socket_type text,
        bios text,
    );"#
}