fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This is a build script for a Rust project that uses the `cc` crate to compile C code.
    // It specifies the source files and includes directories for the C code compilation.
    tonic_build::configure()
        .build_server(true)
        .compile_protos(
            &["proto/services.proto"], 
            &["proto"],
        )?;
    Ok(())
}