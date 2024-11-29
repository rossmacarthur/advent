use std::env;

fn main() {
    let raw = env::var("PROFILE");
    let profile = raw.as_deref().unwrap_or("debug");
    println!("cargo:rerun-if-env-changed=PROFILE");
    println!("cargo:rustc-check-cfg=cfg(profile, values(\"release\", \"{profile}\"))");
    println!("cargo:rustc-cfg=profile=\"{profile}\"");
}
