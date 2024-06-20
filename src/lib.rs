#![feature(
    arbitrary_self_types,
    array_chunks,
    array_windows,
    as_array_of_cells,
    cell_update,
    const_refs_to_static,
    generic_arg_infer,
    generic_const_exprs,
    let_chains,
    maybe_uninit_fill,
    maybe_uninit_uninit_array,
    slice_as_chunks,
    slice_ptr_get,
    slice_take,
    split_array
)]
#![allow(clippy::too_many_arguments, incomplete_features)]
#![warn(clippy::approx_constant)]
#![deny(dead_code)]

mod aac;
mod audio_frame_queue;
mod mpeg4audio_sample_rates;
mod psy_model;
mod sinewin;

mod common;
mod types;

mod avutil;

mod bessel;
