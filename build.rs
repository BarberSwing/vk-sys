use ::std::env;
use ::std::path::PathBuf;

fn main() {
    if let Some(vulkan_sdk) = env::var_os(r"VULKAN_SDK") {
        let mut path = PathBuf::from(vulkan_sdk);
        path.push(r"Lib");

        println!("cargo:rustc-link-search={}", path.display());
    }
}
