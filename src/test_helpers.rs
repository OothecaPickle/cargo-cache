// Copyright 2017-2018 Matthias Krüger. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::path::PathBuf;

#[allow(dead_code)]
pub(crate) fn bin_path() -> String {
    if PathBuf::from("target/release/cargo-cache").is_file() {
        String::from("target/release/cargo-cache")
    } else if PathBuf::from("target/debug/cargo-cache").is_file() {
        String::from("target/debug/cargo-cache")
    } else {
        panic!("No cargo-cache executable found!");
    }
}
