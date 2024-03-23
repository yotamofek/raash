#![feature(
    extern_types,
    const_for,
    const_trait_impl,
    const_mut_refs,
    let_chains,
    c_str_literals,
    array_chunks,
    inline_const,
    generic_const_exprs,
    ptr_from_ref,
    generic_arg_infer,
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

mod aaccoder;
mod aacenc;
mod aacenc_is;
mod aacenc_ltp;
mod aacenc_pred;
mod aacenc_tns;
mod aacenctab;
mod aacpsy;
mod aactab;
mod audio_frame_queue;
mod kbdwin;
mod mpeg4audio_sample_rates;
mod psymodel;
mod sinewin;

mod common;
mod types;

mod avutil;

mod bessel;
