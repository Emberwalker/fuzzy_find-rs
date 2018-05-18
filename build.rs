// Based on https://stackoverflow.com/a/37680212
// Automatically enables the "nightly" feature if built with Nightly - used for enabling bench tests.

extern crate rustc_version;

use rustc_version::{version_meta, Channel};

fn main() {
    if version_meta().channel == Channel::Nightly {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }
}