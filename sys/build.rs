use std::env;

pub fn main() {
    match env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Missing build architecture")
        .as_str()
    {
        "i386" => cargo_emit::rustc_link_lib!("xcapi32"),
        "x86_64" => cargo_emit::rustc_link_lib!("xcapi64"),
        _ => panic!("Invalid build architecture"),
    }
}
