// build.rs
use rustc_version as rs_v;

fn main() {
    // global_allocator is stable since 1.28.0.
    // default global allocator is system allocator sine 1.32.0.
    let rt_version = rs_v::version().unwrap();
    if rt_version >= rs_v::Version::parse("1.32.0").unwrap() {
        // nothing todo
    } else if rt_version >= rs_v::Version::parse("1.28.0").unwrap() {
        println!("cargo:rustc-cfg=has_std_alloc");
    } else if let rs_v::Channel::Nightly = rs_v::version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=rustc_nightly");
        println!("cargo:rustc-cfg=has_global_allocator");
    }
    //
    if rt_version < rs_v::Version::parse("1.42.0").unwrap() {
        println!("cargo:rustc-cfg=has_not_matches");
    }
}
