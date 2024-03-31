use std::{cmp::Ordering, iter::zip};

use itertools::Itertools as _;
use libc::{c_float, c_int, c_uint};

use super::{aac_cb_in_map, aac_cb_out_map, put_bits, quantize_band_cost_bits, run_value_bits};
use crate::{
    aac::encoder::{ctx::AACEncContext, pow::Pow34 as _},
    types::{BandType, BitBuf, SingleChannelElement},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct BandCodingPath {
    pub prev_idx: c_int,
    pub cost: c_float,
    pub run: c_int,
}

impl BandCodingPath {
    /// (yotam):
    /// Compare the cost of this path vs. another path by [total
    /// ordering](c_float::total_cmp).
    fn cost_cmp(&self, other: &Self) -> Ordering {
        self.cost.total_cmp(&other.cost)
    }
}

pub(crate) unsafe fn codebook_rate(
    s: *mut AACEncContext,
    sce: *mut SingleChannelElement,
    win: c_int,
    group_len: c_int,
) {
    let mut path = [[BandCodingPath::default(); 15]; 120];
    let max_sfb: c_int = (*sce).ics.max_sfb as c_int;
    let run_bits: c_int = if (*sce).ics.num_windows == 1 { 5 } else { 3 };
    let run_esc: c_int = (1 << run_bits) - 1;
    let mut ppos: c_int = 0;
    let mut next_minbits = c_float::INFINITY;
    let mut next_mincb: c_int = 0;
    for (scoef, coef) in zip(&mut (*s).scoefs, &(*sce).coeffs) {
        *scoef = coef.abs_pow34();
    }
    let mut start = win * 128;
    path[0].fill(BandCodingPath {
        prev_idx: -1,
        cost: (run_bits + 4) as c_float,
        run: 0,
    });
    for swb in 0..max_sfb {
        let size = ((*sce).ics.swb_sizes)[swb as usize] as c_int;
        if (*sce).zeroes[(win * 16 + swb) as usize] {
            let mut cost_stay_here: c_float = path[swb as usize][0].cost;
            let cost_get_here: c_float = next_minbits + run_bits as c_float + 4 as c_float;
            let run_value_bits = run_value_bits((*sce).ics.num_windows);
            if run_value_bits[path[swb as usize][0].run as usize]
                != run_value_bits[(path[swb as usize][0].run + 1) as usize]
            {
                cost_stay_here += run_bits as c_float;
            }
            path[(swb + 1) as usize][0] = if cost_get_here < cost_stay_here {
                BandCodingPath {
                    prev_idx: next_mincb,
                    cost: cost_get_here,
                    run: 1,
                }
            } else {
                BandCodingPath {
                    prev_idx: 0,
                    cost: cost_stay_here,
                    run: path[swb as usize][0].run + 1,
                }
            };
            next_minbits = path[(swb + 1) as usize][0].cost;
            next_mincb = 0;
            path[(swb + 1) as usize][1..].fill(BandCodingPath {
                prev_idx: -1,
                cost: 61450.,
                run: 0,
            });
        } else {
            let minbits: c_float = next_minbits;
            let mincb: c_int = next_mincb;
            let startcb =
                aac_cb_in_map[(*sce).band_type[(win * 16 + swb) as usize] as usize] as c_int;
            next_minbits = ::core::f32::INFINITY;
            next_mincb = 0;
            path[(swb + 1) as usize].fill(BandCodingPath {
                prev_idx: -1,
                cost: 61450.,
                run: 0,
            });
            for cb in startcb..15 {
                if cb >= 12
                    && (*sce).band_type[(win * 16 + swb) as usize]
                        != c_uint::from(aac_cb_out_map[cb as usize])
                {
                    path[(swb + 1) as usize][cb as usize] = BandCodingPath {
                        prev_idx: -1,
                        cost: 61450.,
                        run: 0,
                    };
                } else {
                    let bits = (0..group_len)
                        .map(|w| {
                            quantize_band_cost_bits(
                                s,
                                (*sce).coeffs[(start + w * 128) as usize..].as_ptr(),
                                (*s).scoefs[(start + w * 128) as usize..].as_ptr(),
                                size,
                                (*sce).sf_idx[(win * 16 + swb) as usize],
                                aac_cb_out_map[cb as usize] as c_int,
                                f32::INFINITY,
                            ) as c_float
                        })
                        .sum::<c_float>();

                    let mut cost_stay_here_0 = path[swb as usize][cb as usize].cost + bits;
                    let cost_get_here_0 = minbits + bits + run_bits as c_float + 4 as c_float;
                    let run_value_bits = run_value_bits((*sce).ics.num_windows);
                    if run_value_bits[path[swb as usize][cb as usize].run as usize]
                        != run_value_bits[(path[swb as usize][cb as usize].run + 1) as usize]
                    {
                        cost_stay_here_0 += run_bits as c_float;
                    }
                    path[(swb + 1) as usize][cb as usize] = if cost_get_here_0 < cost_stay_here_0 {
                        BandCodingPath {
                            prev_idx: mincb,
                            cost: cost_get_here_0,
                            run: 1,
                        }
                    } else {
                        BandCodingPath {
                            prev_idx: cb,
                            cost: cost_stay_here_0,
                            run: path[swb as usize][cb as usize].run + 1,
                        }
                    };
                    if path[(swb + 1) as usize][cb as usize].cost < next_minbits {
                        next_minbits = path[(swb + 1) as usize][cb as usize].cost;
                        next_mincb = cb;
                    }
                }
            }
        }
        start += (*sce).ics.swb_sizes[swb as usize] as c_int;
    }

    let mut idx = path[max_sfb as usize]
        .iter()
        .position_min_by(|a, b| BandCodingPath::cost_cmp(a, b))
        .unwrap() as c_int;

    let mut stack_len = 0;
    let mut stackrun: [c_int; 120] = [0; 120];
    let mut stackcb: [c_int; 120] = [0; 120];

    ppos = max_sfb;
    while ppos > 0 {
        let cb = idx;
        stackrun[stack_len as usize] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len as usize] = cb;
        idx =
            path[(ppos - path[ppos as usize][cb as usize].run + 1) as usize][cb as usize].prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
    }
    let mut start = 0;
    for i in (0..stack_len).rev() {
        let cb = aac_cb_out_map[stackcb[i as usize] as usize] as c_int;
        put_bits(&mut (*s).pb, 4, cb as BitBuf);
        let mut count = stackrun[i as usize];
        (*sce).zeroes[(win * 16 + start) as usize..][..count as usize].fill(cb == 0);
        (*sce).band_type[(win * 16 + start) as usize..][..count as usize].fill(cb as BandType);
        start += count;
        while count >= run_esc {
            put_bits(&mut (*s).pb, run_bits, run_esc as BitBuf);
            count -= run_esc;
        }
        put_bits(&mut (*s).pb, run_bits, count as BitBuf);
    }
}
