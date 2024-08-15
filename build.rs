use ::std::env;
use ::std::path::PathBuf;

fn main() {
    let vulkan_sdk = env::var_os(r"VULKAN_SDK").unwrap();

    let mut path = PathBuf::from(vulkan_sdk);
    path.push(r"Lib");

    println!("cargo:rustc-link-search={}", path.display());
}
