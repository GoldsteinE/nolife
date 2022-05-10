//! Static memory management without lifetimes.
//!
//! You need to specify `#![feature(generic_const_exprs)]` in your crate for this to work.
//!
//! ```
//! # #![feature(generic_const_exprs)]
//! # use nolife::*;
//! let owned = heap!(0);
//! let (husk, mut reference) = borrow!(owned);
//! // This reference is mutable:
//! *reference += 1;
//! let [ref1, ref2] = reference.split();
//! // These references work:
//! assert_eq!(*ref1, *ref2);
//! assert_eq!(*ref1, 1);
//! # let _ = husk;
//! ```
//! References obtained by [`.split()`](Ref::split) are no longer mutable:
//! ```compile_fail
//! # use nolife::*;
//! # let (_husk, mut reference) = borrow!(heap!(0));
//! # let [ref1, _ref2] = reference.split();
//! *ref1 += 1;
//! ```
//! We can join them together to make a mutable reference again:
//! ```
//! # #![feature(generic_const_exprs)]
//! # use nolife::*;
//! # let (_husk, mut reference) = borrow!(heap!(0));
//! # let [ref1, ref2] = reference.split();
//! let mut reference = ref1.join(ref2);
//! *reference += 1;
//! ```
//! And then reconstruct an owned value:
//! ```
//! # #![feature(generic_const_exprs)]
//! # use nolife::*;
//! # let (husk, mut reference) = borrow!(heap!(0));
//! let owned = reference.reconstruct(husk);
//! ```
//! And even obtain an ownership over the contained value again:
//! ```
//! # use nolife::*;
//! # let owned = heap!(0);
//! let truly_owned = owned.into_inner();
//! fn assert_type_is_i32(_: i32) {}
//! assert_type_is_i32(truly_owned);
//! ```
//! Trying to join two references obtained from different owned values is a compilation error:
//! ```compile_fail
//! # #![feature(generic_const_exprs)]
//! # use nolife::*;
//! let (husk1, ref1) = borrow!(heap!(0));
//! let (husk2, ref2) = borrow!(heap!(0));
//! let [ref11, ref12] = ref1.split();
//! let [ref21, ref22] = ref2.split();
//! ref12.join(ref22);
//! ```
//! Trying to reconstruct an owned value using reference and husk obtained from different owned
//! values is also a compilation error:
//! ```compile_fail
//! let (husk1, ref1) = borrow!(heap!(0));
//! let (husk2, ref2) = borrow!(heap!(0));
//! ref2.reconstruct(husk1);
//! ```
   
#![allow(incomplete_features, dead_code, unused_unsafe)]
#![warn(clippy::pedantic)]
#![feature(generic_const_exprs)]
#![cfg_attr(feature = "const_string_brands", feature(adt_const_params))]
// lint me harder
#![forbid(non_ascii_idents)]
#![deny(keyword_idents)]
#![deny(elided_lifetimes_in_paths)]
#![deny(rust_2021_incompatible_closure_captures)]
#![deny(rust_2021_incompatible_or_patterns)]
#![deny(rust_2021_prefixes_incompatible_syntax)]
#![warn(explicit_outlives_requirements)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
#![warn(rust_2021_prelude_collisions)]
#![warn(semicolon_in_expressions_from_macros)]
#![warn(single_use_lifetimes)]
#![warn(trivial_numeric_casts)]
#![warn(unused_crate_dependencies)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![deny(clippy::fallible_impl_from)]
#![deny(clippy::wildcard_dependencies)]
#![warn(clippy::pedantic)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::debug_assert_with_mut_call)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::exit)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::option_if_let_else)]
#![warn(clippy::panic)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_lit_as_bytes)]
#![warn(clippy::string_to_string)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::useless_let_if_seq)]
#![allow(clippy::missing_errors_doc)]

pub mod brand;

mod owned;
pub use owned::{Heap, Owned, OwnershipKind, Husk};

mod reference;
pub use reference::{Ref, RefMut};
