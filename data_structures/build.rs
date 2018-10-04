use std::env;
use std::process::Command;

fn main() {
    let schemas_path = "../schemas";
    let out_path = env::var("OUT_DIR").unwrap();

    let protocol_schemas = &format!("{}/protocol.fbs", schemas_path);

    // generate protocol with flatc
    let out_put = Command::new("flatc")
            .args(&["-r", "-o", &out_path, protocol_schemas])
            .output()
            .expect("Failed to generate protocol");
    if !out_put.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&out_put.stderr));
    }
}
