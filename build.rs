use std::env;
use homedir;

type AbstractError = Box<dyn std::error::Error>;

fn main() -> Result<(), AbstractError> {
    if cfg!(windows) && env::var("PROTOC").is_err() {
        let u_name = env::var("USERNAME").unwrap();
        let home = homedir::get_home(u_name).unwrap().ok_or("Cannot find homedir").unwrap();
        let home = home.to_str().unwrap();
        env::set_var("PROTOC", format!("{home}\\.local\\bin\\protoc.exe"));
    }
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}
