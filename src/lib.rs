#![feature(
    array_chunks,
    const_for,
    const_mut_refs,
    const_trait_impl,
    extern_types,
    generic_arg_infer,
    generic_const_exprs,
    inline_const,
    let_chains,
    new_uninit,
    slice_take
)]
#![allow(
    clippy::eq_op,
    clippy::precedence,
    clippy::nonminimal_bool,
    clippy::no_effect,
    path_statements,
    clippy::unnecessary_mut_passed,
    clippy::too_many_arguments,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![warn(clippy::approx_constant)]
#![deny(dead_code)]

mod aac;
mod audio_frame_queue;
mod mpeg4audio_sample_rates;
mod psy_model;
mod sinewin;

mod common;
mod types;

mod array;

mod avutil;

mod bessel;
