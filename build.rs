// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use cargo_lock::{Lockfile, Package};
use lazy_static::lazy_static;

/// Set an environment variable `<PKG>_VERSION` with the version number of `pkg`.
fn emit_dep_ver(pkg: &str) {
    lazy_static! {
        static ref LOCKFILE: Lockfile = Lockfile::load("Cargo.lock").unwrap();
        static ref THIS_PKG: &'static Package = LOCKFILE
            .packages
            .iter()
            .find(|pkg| pkg.name.as_str() == env!("CARGO_PKG_NAME"))
            .unwrap();
    }

    let pkg_ver = &THIS_PKG
        .dependencies
        .iter()
        .find(|&dep| dep.name.as_str() == pkg)
        .unwrap()
        .version;

    println!("cargo:rustc-env={}_VERSION={}", pkg.to_uppercase(), pkg_ver);
}

fn main() {
    emit_dep_ver("ureq");
    emit_dep_ver("reqwest");
}
