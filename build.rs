use cargo_lock::{Lockfile, Package, Version};

/// Look for the version of crate `dep_name` from our dependencies.
fn dep_ver<'a>(this_pkg: &'a Package, dep_name: &'a str) -> Option<&'a Version> {
    this_pkg
        .dependencies
        .iter()
        .find(|dep| dep.name.as_str() == dep_name)
        .map(|dep| &dep.version)
}

fn main() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let this_pkg = lockfile
        .packages
        .iter()
        .find(|pkg| pkg.name.as_str() == env!("CARGO_PKG_NAME"))
        .unwrap();

    ["ureq", "reqwest", "curl"]
        .into_iter()
        .for_each(|dep_name| {
            if let Some(version) = dep_ver(this_pkg, dep_name) {
                println!(
                    "cargo:rustc-env={}_VERSION={}",
                    dep_name.to_uppercase(),
                    version
                );
            }
        });
}
