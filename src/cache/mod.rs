// Copyright 2017-2018 Matthias Krüger. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub(crate) mod dircache;

pub(crate) mod bin;
pub(crate) mod git_checkouts;
pub(crate) mod git_repos_bare;
pub(crate) mod registry_cache;
pub(crate) mod registry_index;
pub(crate) mod registry_sources;

// The idea of this module is to be a sort of cache
// once a value is first asked for, we calculate the value, save it and return it
// if queried again, simply return the saved value.
