// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_ty_one"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_cty_base as cty_base;
extern crate linux_ty_zero as ty_zero;
extern crate linux_arch_fns as arch_fns;

pub use ty_zero::{clone, result, error};

pub mod linux {
    pub use ::core::linux::*;
    pub use {clone, result};
}

pub mod bytes;
pub mod byte_str;
pub mod copy_cell;
pub mod c_str;
pub mod iter_ext;
pub mod num;
pub mod parse;
pub mod path;
pub mod range;
pub mod ref_cell;
pub mod rmo;
pub mod saturating_cast;

pub mod prelude {
    pub use ty_zero::prelude::*;
    pub use num::{UnsignedInt};
}