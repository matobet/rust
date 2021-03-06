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
//!   provided by the [compiler-builtins crate](https://crates.io/crates/compiler_builtins).
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
// and this will cause the E0152 "found duplicate lang item" error. See
// discussion in #50466 for details.
//
// This cfg won't affect doc tests.
#![cfg(not(test))]
#![stable(feature = "core", since = "1.6.0")]
#![doc(
    html_root_url = "https://doc.rust-lang.org/nightly/",
    html_playground_url = "https://play.rust-lang.org/",
    issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]
#![no_core]
#![warn(deprecated_in_future)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
#![allow(explicit_outlives_requirements)]
#![allow(incomplete_features)]
#![feature(allow_internal_unstable)]
#![feature(arbitrary_self_types)]
#![feature(asm)]
#![feature(bound_cloned)]
#![feature(cfg_target_has_atomic)]
#![feature(concat_idents)]
#![feature(const_ascii_ctype_on_intrinsics)]
#![feature(const_alloc_layout)]
#![feature(const_discriminant)]
#![cfg_attr(bootstrap, feature(const_if_match))]
#![cfg_attr(bootstrap, feature(const_loop))]
#![feature(const_checked_int_methods)]
#![feature(const_euclidean_int_methods)]
#![feature(const_overflowing_int_methods)]
#![feature(const_saturating_int_methods)]
#![feature(const_int_unchecked_arith)]
#![feature(const_int_pow)]
#![feature(constctlz)]
#![feature(const_panic)]
#![feature(const_fn_union)]
#![feature(const_generics)]
#![feature(const_ptr_offset)]
#![feature(const_ptr_offset_from)]
#![cfg_attr(not(bootstrap), feature(const_raw_ptr_comparison))]
#![feature(const_result)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_slice_ptr_len)]
#![feature(const_type_name)]
#![feature(const_likely)]
#![feature(custom_inner_attributes)]
#![feature(decl_macro)]
#![feature(doc_cfg)]
#![feature(extern_types)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(try_find)]
#![feature(is_sorted)]
#![feature(lang_items)]
#![feature(link_llvm_intrinsics)]
#![feature(llvm_asm)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(nll)]
#![feature(exhaustive_patterns)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(or_patterns)]
#![feature(prelude_import)]
#![feature(repr_simd, platform_intrinsics)]
#![feature(rustc_attrs)]
#![feature(simd_ffi)]
#![feature(specialization)]
#![feature(staged_api)]
#![feature(std_internals)]
#![feature(stmt_expr_attributes)]
#![cfg_attr(bootstrap, feature(track_caller))]
#![feature(transparent_unions)]
#![feature(unboxed_closures)]
#![feature(unsized_locals)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![cfg_attr(not(bootstrap), feature(variant_count))]
#![feature(doc_alias)]
#![feature(mmx_target_feature)]
#![feature(tbm_target_feature)]
#![feature(sse4a_target_feature)]
#![feature(arm_target_feature)]
#![feature(powerpc_target_feature)]
#![feature(mips_target_feature)]
#![feature(aarch64_target_feature)]
#![feature(wasm_target_feature)]
#![feature(avx512_target_feature)]
#![feature(cmpxchg16b_target_feature)]
#![feature(rtm_target_feature)]
#![feature(f16c_target_feature)]
#![feature(hexagon_target_feature)]
#![feature(const_transmute)]
#![feature(abi_unadjusted)]
#![feature(adx_target_feature)]
#![feature(maybe_uninit_slice)]
#![feature(external_doc)]
#![feature(associated_type_bounds)]
#![feature(const_type_id)]
#![feature(const_caller_location)]
#![feature(no_niche)] // rust-lang/rust#68303
#![feature(unsafe_block_in_unsafe_fn)]
#![deny(unsafe_op_in_unsafe_fn)]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[cfg(not(test))] // See #65860
#[macro_use]
mod macros;

#[macro_use]
mod internal_macros;

#[path = "num/int_macros.rs"]
#[macro_use]
mod int_macros;

#[path = "num/i128.rs"]
pub mod i128;
#[path = "num/i16.rs"]
pub mod i16;
#[path = "num/i32.rs"]
pub mod i32;
#[path = "num/i64.rs"]
pub mod i64;
#[path = "num/i8.rs"]
pub mod i8;
#[path = "num/isize.rs"]
pub mod isize;

#[path = "num/u128.rs"]
pub mod u128;
#[path = "num/u16.rs"]
pub mod u16;
#[path = "num/u32.rs"]
pub mod u32;
#[path = "num/u64.rs"]
pub mod u64;
#[path = "num/u8.rs"]
pub mod u8;
#[path = "num/usize.rs"]
pub mod usize;

#[path = "num/f32.rs"]
pub mod f32;
#[path = "num/f64.rs"]
pub mod f64;

#[macro_use]
pub mod num;

/* The libcore prelude, not as all-encompassing as the libstd prelude */

pub mod prelude;

/* Core modules for ownership management */

pub mod hint;
pub mod intrinsics;
pub mod mem;
pub mod ptr;

/* Core language traits */

pub mod borrow;
#[cfg(not(test))] // See #65860
pub mod clone;
#[cfg(not(test))] // See #65860
pub mod cmp;
pub mod convert;
#[cfg(not(test))] // See #65860
pub mod default;
#[cfg(not(test))] // See #65860
pub mod marker;
pub mod ops;

/* Core types and methods on primitives */

pub mod any;
#[cfg(not(test))] // See #65860
pub mod array;
pub mod ascii;
pub mod cell;
pub mod char;
pub mod ffi;
#[cfg(not(test))] // See #65860
pub mod iter;
pub mod option;
pub mod panic;
pub mod panicking;
#[cfg(not(test))] // See #65860
pub mod pin;
pub mod raw;
pub mod result;
pub mod sync;

#[cfg(not(test))] // See #65860
pub mod fmt;
#[cfg(not(test))] // See #65860
pub mod hash;
pub mod slice;
#[cfg(not(test))] // See #65860
pub mod str;
pub mod time;

pub mod unicode;

/* Async */
#[cfg(not(test))] // See #65860
pub mod future;
pub mod task;

/* Heap memory allocator trait */
#[allow(missing_docs)]
pub mod alloc;

// note: does not need to be public
mod bool;
mod tuple;
mod unit;

#[stable(feature = "core_primitive", since = "1.43.0")]
pub mod primitive;

// Pull in the `core_arch` crate directly into libcore. The contents of
// `core_arch` are in a different repository: rust-lang/stdarch.
//
// `core_arch` depends on libcore, but the contents of this module are
// set up in such a way that directly pulling it here works such that the
// crate uses the this crate as its libcore.
#[path = "../stdarch/crates/core_arch/src/mod.rs"]
#[allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    unused_imports,
    unsafe_op_in_unsafe_fn
)]
// FIXME: This annotation should be moved into rust-lang/stdarch after clashing_extern_declarations is
// merged. It currently cannot because bootstrap fails as the lint hasn't been defined yet.
#[cfg_attr(not(bootstrap), allow(clashing_extern_declarations))]
#[unstable(feature = "stdsimd", issue = "48556")]
mod core_arch;

#[stable(feature = "simd_arch", since = "1.27.0")]
pub use core_arch::arch;
