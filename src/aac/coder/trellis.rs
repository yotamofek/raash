use std::{cmp::Ordering, iter::zip};

use array_util::{Array, WindowedArray, W, W2};
use bit_writer::{BitBuf, BitWriter};
use ffmpeg_src_macro::ffmpeg_src;
use itertools::Itertools as _;
use izip::izip;
use libc::{c_float, c_int, c_uint, c_ushort};

use super::{quantize_band_cost_bits, run_value_bits, WindowCount, CB_IN_MAP, CB_OUT_MAP};
use crate::{
    aac::encoder::{ctx::AACEncContext, pow::Pow34 as _},
    types::{BandType, SingleChannelElement},
};

/// structure used in optimal codebook search
#[ffmpeg_src(file = "libavcodec/aaccoder_trellis.h", lines = 49..=56, name = "TrellisBandCodingPath")]
#[derive(Copy, Clone, Debug, Default)]
struct BandCodingPath {
    /// pointer to the previous path point
    prev_idx: c_int,
    /// path cost
    cost: c_float,
    run: c_int,
}

impl BandCodingPath {
    /// (yotam):
    /// Compare the cost of this path vs. another path by [total
    /// ordering](c_float::total_cmp).
    fn cost_cmp(&self, other: &Self) -> Ordering {
        self.cost.total_cmp(&other.cost)
    }
}

fn calc_path(
    sce: &SingleChannelElement,
    scaled_coeffs: &WindowedArray<Array<f32, 1024>, 128>,
    win: i32,
    run_bits: u8,
    group_len: i32,
) -> [[BandCodingPath; 15]; 120] {
    const ZERO_PATH: BandCodingPath = BandCodingPath {
        prev_idx: -1,
        cost: 61450.,
        run: 0,
    };

    let mut path = [[BandCodingPath::default(); 15]; 120];

    path[0].fill(BandCodingPath {
        prev_idx: -1,
        cost: (run_bits + 4).into(),
        run: 0,
    });

    let mut next_minbits = c_float::INFINITY;
    let mut next_mincb = 0;
    let mut start = 0;

    let mut cur_path = path.as_mut_slice();
    for (&swb_size, &zero, &band_type, &sf_idx) in izip!(
        sce.ics.swb_sizes,
        &sce.zeroes[W(win)],
        &sce.band_type[W(win)],
        &sce.sf_idx[W(win)],
    )
    .take(sce.ics.max_sfb.into())
    {
        let [ref path0, path1] = cur_path.first_chunk_mut().unwrap();

        if zero {
            let [BandCodingPath { cost, run, .. }, ..] = *path0;

            let mut cost_stay_here = cost;
            let cost_get_here = next_minbits + c_float::from(run_bits) + 4.;
            let run_value_bits = run_value_bits(sce.ics.num_windows);
            if run_value_bits[run as usize] != run_value_bits[run as usize + 1] {
                cost_stay_here += c_float::from(run_bits);
            }

            let [path1_0, path1 @ ..] = path1;
            *path1_0 = if cost_get_here < cost_stay_here {
                BandCodingPath {
                    prev_idx: next_mincb,
                    cost: cost_get_here,
                    run: 1,
                }
            } else {
                BandCodingPath {
                    prev_idx: 0,
                    cost: cost_stay_here,
                    run: run + 1,
                }
            };
            next_minbits = path1_0.cost;
            next_mincb = 0;
            path1.fill(ZERO_PATH);
        } else {
            let minbits = next_minbits;
            let mincb = next_mincb;
            let startcb = CB_IN_MAP[band_type as usize];
            next_minbits = f32::INFINITY;
            next_mincb = 0;
            path1.fill(ZERO_PATH);
            for (cb, &cb_out, path0, path1) in izip!(
                startcb..15,
                &CB_OUT_MAP[startcb.into()..],
                &path0[startcb.into()..],
                &mut path1[startcb.into()..]
            ) {
                if cb >= 12 && band_type != c_uint::from(cb_out) {
                    *path1 = ZERO_PATH;
                    continue;
                }

                let bits = izip!(&sce.coeffs[W2(win)], &scaled_coeffs[W2(win)])
                    .take(group_len as usize)
                    .map(|(coeffs, scaled)| {
                        quantize_band_cost_bits(
                            &coeffs[start.into()..][..swb_size.into()],
                            &scaled[start.into()..][..swb_size.into()],
                            sf_idx,
                            cb_out.into(),
                            f32::INFINITY,
                        )
                    })
                    .sum::<c_int>() as c_float;

                let mut cost_stay_here = path0.cost + bits;
                let cost_get_here = minbits + bits + c_float::from(run_bits) + 4.;
                let run_value_bits = run_value_bits(sce.ics.num_windows);
                if run_value_bits[path0.run as usize] != run_value_bits[path0.run as usize + 1] {
                    cost_stay_here += c_float::from(run_bits);
                }
                *path1 = if cost_get_here < cost_stay_here {
                    BandCodingPath {
                        prev_idx: mincb,
                        cost: cost_get_here,
                        run: 1,
                    }
                } else {
                    BandCodingPath {
                        prev_idx: cb.into(),
                        cost: cost_stay_here,
                        run: path0.run + 1,
                    }
                };
                if path1.cost < next_minbits {
                    next_minbits = path1.cost;
                    next_mincb = cb.into();
                }
            }
        }

        start += c_ushort::from(swb_size);
        cur_path = &mut cur_path[1..];
    }

    path
}

#[ffmpeg_src(file = "libavcodec/aaccoder_trellis.h", lines = 59..=189, name = "codebook_trellis_rate")]
pub(crate) fn codebook_rate(
    s: &mut AACEncContext,
    sce: &mut SingleChannelElement,
    pb: &mut BitWriter,
    win: c_int,
    group_len: c_int,
) {
    let max_sfb = c_int::from(sce.ics.max_sfb);
    let run_bits = if sce.ics.num_windows == WindowCount::One {
        5
    } else {
        3
    };
    let run_esc = (1 << run_bits) - 1;

    for (scoef, coef) in zip(&mut *s.scaled_coeffs, &*sce.coeffs) {
        *scoef = coef.abs_pow34();
    }

    let path = calc_path(sce, &s.scaled_coeffs, win, run_bits, group_len);

    // convert resulting path from backward-linked list
    let mut idx = path[max_sfb as usize]
        .iter()
        .position_min_by(|a, b| BandCodingPath::cost_cmp(a, b))
        .unwrap() as c_int;
    let mut stack_len = 0;
    let mut stackrun: [c_int; 120] = [0; 120];
    let mut stackcb: [c_int; 120] = [0; 120];
    let mut ppos = max_sfb;
    while ppos > 0 {
        let cb = idx;
        stackrun[stack_len] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len] = cb;
        idx =
            path[(ppos - path[ppos as usize][cb as usize].run + 1) as usize][cb as usize].prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
    }

    // perform actual band info encoding
    let mut zeroes = &mut sce.zeroes[W(win)];
    let mut band_type = &mut sce.band_type[W(win)];
    for (&stackcb, &stackrun) in zip(&stackcb, &stackrun).take(stack_len).rev() {
        let cb = CB_OUT_MAP[stackcb as usize];
        pb.put(4, BitBuf::from(cb));

        zeroes.take_mut(..stackrun as usize).unwrap().fill(cb == 0);
        band_type
            .take_mut(..stackrun as usize)
            .unwrap()
            .fill(cb as BandType);

        let mut count = stackrun;
        while count >= run_esc {
            pb.put(run_bits, run_esc as BitBuf);
            count -= run_esc;
        }
        pb.put(run_bits, count as BitBuf);
    }
}
