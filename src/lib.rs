#![feature(extern_types, c_str_literals)]
#![allow(
    clippy::eq_op,
    clippy::precedence,
    clippy::nonminimal_bool,
    clippy::no_effect,
    path_statements,
    clippy::unnecessary_mut_passed
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
mod common;
mod iirfilter;
mod kbdwin;
mod lpc;
mod mpeg4audio_sample_rates;
mod psymodel;
mod sinewin;
mod types;

mod avutil;
