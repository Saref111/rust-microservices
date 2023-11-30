use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("PROTOC", format!("C:\\Users\\Ivan_Borovyk\\.local\\bin\\protoc.exe"));
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}
