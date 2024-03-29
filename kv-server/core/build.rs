use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .compile_protos(&["command.proto"], &["src/command"])?;
    Ok(())
}