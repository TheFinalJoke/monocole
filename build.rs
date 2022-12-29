// Builder Configs Documentation
// https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compile protocol buffer using protoc
    tonic_build::configure()
        .build_server(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &[
                "protos/config/config.proto",
                "protos/controller/control.proto",
                "protos/hardware/hardware.proto",
                "protos/os/linux.proto",
            ],
            &["protos"],
        )?;
    Ok(())
}
