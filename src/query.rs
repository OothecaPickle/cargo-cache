// Copyright 2017-2019 Matthias Krüger. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::path::PathBuf;

use crate::cache::dircache::Cache;
use crate::cache::*;
use crate::library::CargoCachePaths;
use crate::top_items::binaries::*;
use crate::top_items::git_checkouts::*;
use crate::top_items::git_repos_bare::*;
use crate::top_items::registry_cache::*;
use crate::top_items::registry_sources::*;

use clap::ArgMatches;
use regex::Regex;

#[derive(Debug)]
struct file {
    path: std::path::PathBuf,
    name: std::string::String,
    size: u64,
}

impl file {}

fn binary_to_file(path: std::path::PathBuf) -> file {
    file {
        path: path.clone(),
        name: path
            .clone()
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap_or(String::new()),
        size: fs::metadata(path.clone())
            .unwrap_or_else(|_| panic!("Failed to get metadata of file '{}'", &path.display()))
            .len(),
    }
}

fn sort_files_by_name(v: &mut Vec<&file>) {
    v.sort_by_key(|f| &f.name);
}

fn sort_files_by_size(v: &mut Vec<&file>) {
    v.sort_by_key(|f| &f.size);
}
pub(crate) fn run_query(
    query_config: &ArgMatches<'_>,
    ccd: &CargoCachePaths,
    mut bin_cache: &mut bin::BinaryCache,
    mut checkouts_cache: &mut git_checkouts::GitCheckoutCache,
    mut bare_repos_cache: &mut git_repos_bare::GitRepoCache,
    mut registry_cache: &mut registry_cache::RegistryCache,
    mut registry_sources_cache: &mut registry_sources::RegistrySourceCache,
) {
    enum SortBy {
        Name, // default
        Size,
        Err,
    }

    let sorting: SortBy;
    if query_config.is_present("name") {
        sorting = SortBy::Name;
    } else if query_config.is_present("size") {
        sorting = SortBy::Size;
    } else if query_config.is_present("name") && query_config.is_present("size") {
        sorting = SortBy::Err;
        panic!("query config 'name' and 'size' both present, this should not happen!");
    } else {
        unreachable!();
    }

    println!("Query works!");
    let query = query_config.value_of("QUERY").unwrap_or("" /* default */);

    let mut binary_files = bin_cache
        .files()
        .into_iter()
        .map(|path| binary_to_file(path.to_path_buf())) // convert the path into a file sturct
        .collect::<Vec<_>>();

    let re = match Regex::new(query) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Query failed to parse regex '{}': '{}'", query, e);
            std::process::exit(10);
        }
    };

    let mut matches = binary_files
        .iter()
        .filter(|f| re.is_match(f.name.as_str()))
        .collect::<Vec<_>>();

    println!("Binaries original : {:?}", matches);

    match sorting {
        SortBy::Name => {
            sort_files_by_name(&mut matches);
            println!(
                "Binaries sorted by name : {:?}",
                matches
                    .clone()
                    .into_iter()
                    .map(|f| &f.name)
                    .collect::<Vec<_>>()
            );
        }

        SortBy::Size => {
            sort_files_by_size(&mut matches);
            println!(
                "Binaries sorted by size : {:?}",
                matches
                    .clone()
                    .into_iter()
                    .map(|f| &f.name)
                    .collect::<Vec<_>>()
            );
        }
        SortBy::Err => {
            unreachable!();
        }
    }




}

// @TODO: make sure these work:
// cargo cache q
// cargo cache query
// cargo-cache q
// cargo-cache query