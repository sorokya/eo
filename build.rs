use npm_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exit_status = NpmEnv::default()
        .set_path(std::path::Path::new("eo_protocol_parser"))
       .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
       .init_env()
       .install(None)
       .run("start export rust")
       .exec()?;

    if !exit_status.success() {
        panic!("npm failed");
    }

    // copy protocol file to src
    std::fs::copy("eo_protocol_parser/output/rust/protocol.rs", "src/protocol.rs").unwrap();

    // format protocol file
    let format_status = std::process::Command::new("rustfmt")
        .arg("src/protocol.rs")
        .status()?;

    if !format_status.success() {
        panic!("rustfmt failed. Please install with 'rustup component add rustfmt' and run 'cargo fmt' manually.");
    }

    Ok(())
}
