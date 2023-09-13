#![no_std]
#![deny(
  clippy::inline_asm_x86_att_syntax,
  clippy::all,
  clippy::pedantic,
  clippy::suspicious,
  clippy::complexity
)]
#![allow(
  unused_macros,
  non_camel_case_types,
  non_upper_case_globals,
  non_snake_case,

  // C specific stuff
  clippy::multiple_unsafe_ops_per_block,
  clippy::not_unsafe_ptr_arg_deref,
  clippy::single_call_fn,
  clippy::cast_sign_loss,
  clippy::cast_possible_truncation,
  clippy::cast_lossless,
  clippy::cast_possible_wrap,
  clippy::items_after_statements,
  clippy::unnecessary_cast,

  clippy::too_many_lines,
  clippy::unreadable_literal,
  clippy::mod_module_files,
  clippy::many_single_char_names,
  clippy::if_not_else,
  clippy::else_if_without_else,
  clippy::inline_asm_x86_intel_syntax,
  clippy::module_name_repetitions,
  clippy::similar_names,
  clippy::comparison_chain,
  clippy::must_use_candidate,
  clippy::missing_trait_methods,

  // Documentation related
  clippy::missing_panics_doc
)]
#![feature(thread_local)]

extern crate alloc as allocator;
extern crate cbitset;
extern crate num_traits;

mod alloc;
mod api;
mod macros;
mod types;

// Export types
pub use types::*;

// C library
pub mod arch;
pub mod start;
pub mod std;
pub mod support;
