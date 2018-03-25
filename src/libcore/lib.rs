// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # The Rust Core Library
//!
//! The Rust Core Library is the dependency-free[^free] foundation of [The
//! Rust Standard Library](../std/index.html). It is the portable glue
//! between the language and its libraries, defining the intrinsic and
//! primitive building blocks of all Rust code. It links to no
//! upstream libraries, no system libraries, and no libc.
//!
//! [^free]: Strictly speaking, there are some symbols which are needed but
//!          they aren't always necessary.
//!
//! The core library is *minimal*: it isn't even aware of heap allocation,
//! nor does it provide concurrency or I/O. These things require
//! platform integration, and this library is platform-agnostic.
//!
//! # How to use the core library
//!
//! Please note that all of these details are currently not considered stable.
//!
// FIXME: Fill me in with more detail when the interface settles
//! This library is built on the assumption of a few existing symbols:
//!
//! * `memcpy`, `memcmp`, `memset` - These are core memory routines which are
//!   often generated by LLVM. Additionally, this library can make explicit
//!   calls to these functions. Their signatures are the same as found in C.
//!   These functions are often provided by the system libc, but can also be
//!   provided by the [rlibc crate](https://crates.io/crates/rlibc).
//!
//! * `rust_begin_panic` - This function takes four arguments, a
//!   `fmt::Arguments`, a `&'static str`, and two `u32`'s. These four arguments
//!   dictate the panic message, the file at which panic was invoked, and the
//!   line and column inside the file. It is up to consumers of this core
//!   library to define this panic function; it is only required to never
//!   return. This requires a `lang` attribute named `panic_impl`.
//!
//! * `rust_eh_personality` - is used by the failure mechanisms of the
//!    compiler. This is often mapped to GCC's personality function, but crates
//!    which do not trigger a panic can be assured that this function is never
//!    called. The `lang` attribute is called `eh_personality`.

// Since libcore defines many fundamental lang items, all tests live in a
// separate crate, libcoretest, to avoid bizarre issues.
//
// Here we explicitly #[cfg]-out this whole crate when testing. If we don't do
// this, both the generated test artifact and the linked libtest (which
// transitively includes libcore) will both define the same set of lang items,
// and this will cause the E0152 "duplicate lang item found" error. See
// discussion in #50466 for details.
//
// This cfg won't affect doc tests.
#![cfg(not(test))]

#![stable(feature = "core", since = "1.6.0")]
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]

#![no_core]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#![feature(allow_internal_unstable)]
#![feature(arbitrary_self_types)]
#![feature(asm)]
#![feature(associated_type_defaults)]
#![feature(cfg_target_has_atomic)]
#![feature(concat_idents)]
#![feature(const_fn)]
#![feature(const_int_ops)]
#![feature(const_fn_union)]
#![feature(custom_attribute)]
#![feature(doc_cfg)]
#![feature(doc_spotlight)]
#![feature(extern_types)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(link_llvm_intrinsics)]
#![feature(never_type)]
#![cfg_attr(not(stage0), feature(nll))]
#![cfg_attr(not(stage0), feature(infer_outlives_requirements))]
#![feature(exhaustive_patterns)]
#![feature(macro_at_most_once_rep)]
#![feature(no_core)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(repr_simd, platform_intrinsics)]
#![feature(rustc_attrs)]
#![feature(rustc_const_unstable)]
#![feature(simd_ffi)]
#![feature(specialization)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
#![feature(unboxed_closures)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(doc_alias)]
#![feature(mmx_target_feature)]
#![feature(tbm_target_feature)]
#![feature(sse4a_target_feature)]
#![feature(arm_target_feature)]
#![feature(powerpc_target_feature)]
#![feature(mips_target_feature)]
#![feature(aarch64_target_feature)]
#![feature(const_slice_len)]
#![feature(const_str_as_bytes)]
#![feature(const_str_len)]
#![feature(non_exhaustive)]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[macro_use]
mod macros;

#[macro_use]
mod internal_macros;

#[path = "num/int_macros.rs"]
#[macro_use]
mod int_macros;

#[path = "num/uint_macros.rs"]
#[macro_use]
mod uint_macros;

#[path = "num/isize.rs"] pub mod isize;
#[path = "num/i8.rs"]    pub mod i8;
#[path = "num/i16.rs"]   pub mod i16;
#[path = "num/i32.rs"]   pub mod i32;
#[path = "num/i64.rs"]   pub mod i64;
#[path = "num/i128.rs"]  pub mod i128;

#[path = "num/usize.rs"] pub mod usize;
#[path = "num/u8.rs"]    pub mod u8;
#[path = "num/u16.rs"]   pub mod u16;
#[path = "num/u32.rs"]   pub mod u32;
#[path = "num/u64.rs"]   pub mod u64;
#[path = "num/u128.rs"]  pub mod u128;

#[path = "num/f32.rs"]   pub mod f32;
#[path = "num/f64.rs"]   pub mod f64;

#[macro_use]
pub mod num;

/* The libcore prelude, not as all-encompassing as the libstd prelude */

pub mod prelude;

/* Core modules for ownership management */

pub mod intrinsics;
pub mod mem;
pub mod ptr;
pub mod hint;

/* Core language traits */

pub mod marker;
pub mod ops;
pub mod cmp;
pub mod clone;
pub mod default;
pub mod convert;
pub mod borrow;

/* Core types and methods on primitives */

pub mod any;
pub mod array;
pub mod ascii;
pub mod sync;
pub mod cell;
pub mod char;
pub mod panic;
pub mod panicking;
pub mod pin;
pub mod iter;
pub mod option;
pub mod raw;
pub mod result;

#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
           issue = "27745")]
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips",
          target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86",
          target_arch = "x86_64"))]
#[cfg(not(stage0))]
pub mod ffi;

pub mod slice;
pub mod str;
pub mod hash;
pub mod fmt;
pub mod time;

pub mod unicode;

/* Async */
pub mod future;
pub mod task;

/* Heap memory allocator trait */
#[allow(missing_docs)]
pub mod alloc;

// note: does not need to be public
mod iter_private;
mod nonzero;
mod tuple;
mod unit;

// Pull in the the `coresimd` crate directly into libcore. This is where all the
// architecture-specific (and vendor-specific) intrinsics are defined. AKA
// things like SIMD and such. Note that the actual source for all this lies in a
// different repository, rust-lang-nursery/stdsimd. That's why the setup here is
// a bit wonky.
#[allow(unused_macros)]
macro_rules! test_v16 { ($item:item) => {}; }
#[allow(unused_macros)]
macro_rules! test_v32 { ($item:item) => {}; }
#[allow(unused_macros)]
macro_rules! test_v64 { ($item:item) => {}; }
#[allow(unused_macros)]
macro_rules! test_v128 { ($item:item) => {}; }
#[allow(unused_macros)]
macro_rules! test_v256 { ($item:item) => {}; }
#[allow(unused_macros)]
macro_rules! test_v512 { ($item:item) => {}; }
#[allow(unused_macros)]
macro_rules! vector_impl { ($([$f:ident, $($args:tt)*]),*) => { $($f!($($args)*);)* } }
#[path = "../stdsimd/coresimd/mod.rs"]
#[allow(missing_docs, missing_debug_implementations, dead_code, unused_imports)]
#[unstable(feature = "stdsimd", issue = "48556")]
#[cfg(not(stage0))] // allow changes to how stdsimd works in stage0
mod coresimd;

#[stable(feature = "simd_arch", since = "1.27.0")]
#[cfg(not(stage0))]
pub use coresimd::arch;
