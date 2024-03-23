use std::ptr;

use libc::{c_float, c_int, c_uint};

use super::{aac_cb_in_map, aac_cb_out_map, put_bits, quantize_band_cost_bits, run_value_bits};
use crate::{
    aacenc::{abs_pow34_v, ctx::AACEncContext},
    types::{BandType, BitBuf, SingleChannelElement},
};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub(super) struct BandCodingPath {
    pub prev_idx: c_int,
    pub cost: c_float,
    pub run: c_int,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub(super) struct Path {
    pub cost: c_float,
    pub prev: c_int,
}

pub(crate) unsafe fn codebook_rate(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut win: c_int,
    mut group_len: c_int,
    _lambda: c_float,
) {
    let mut path = [[BandCodingPath::default(); 15]; 120];
    let mut w: c_int = 0;
    let mut swb: c_int = 0;
    let mut cb: c_int = 0;
    let mut start: c_int = 0;
    let mut size: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let max_sfb: c_int = (*sce).ics.max_sfb as c_int;
    let run_bits: c_int = if (*sce).ics.num_windows == 1 { 5 } else { 3 };
    let run_esc: c_int = ((1) << run_bits) - 1;
    let mut idx: c_int = 0;
    let mut ppos: c_int = 0;
    let mut count: c_int = 0;
    let mut stackrun: [c_int; 120] = [0; 120];
    let mut stackcb: [c_int; 120] = [0; 120];
    let mut stack_len: c_int = 0;
    let mut next_minbits: c_float = ::core::f32::INFINITY;
    let mut next_mincb: c_int = 0;
    abs_pow34_v(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024,
    );
    start = win * 128;
    cb = 0;
    while cb < 15 {
        path[0][cb as usize].cost = (run_bits + 4) as c_float;
        path[0][cb as usize].prev_idx = -1;
        path[0][cb as usize].run = 0;
        cb += 1;
        cb;
    }
    swb = 0;
    while swb < max_sfb {
        size = *((*sce).ics.swb_sizes).offset(swb as isize) as c_int;
        if (*sce).zeroes[(win * 16 + swb) as usize] != 0 {
            let mut cost_stay_here: c_float = path[swb as usize][0].cost;
            let mut cost_get_here: c_float = next_minbits + run_bits as c_float + 4 as c_float;
            if *(run_value_bits[((*sce).ics.num_windows == 8) as c_int as usize])
                .offset(path[swb as usize][0].run as isize) as c_int
                != *(run_value_bits[((*sce).ics.num_windows == 8) as c_int as usize])
                    .offset((path[swb as usize][0].run + 1) as isize) as c_int
            {
                cost_stay_here += run_bits as c_float;
            }
            if cost_get_here < cost_stay_here {
                path[(swb + 1) as usize][0].prev_idx = next_mincb;
                path[(swb + 1) as usize][0].cost = cost_get_here;
                path[(swb + 1) as usize][0].run = 1;
            } else {
                path[(swb + 1) as usize][0].prev_idx = 0;
                path[(swb + 1) as usize][0].cost = cost_stay_here;
                path[(swb + 1) as usize][0].run = path[swb as usize][0].run + 1;
            }
            next_minbits = path[(swb + 1) as usize][0].cost;
            next_mincb = 0;
            cb = 1;
            while cb < 15 {
                path[(swb + 1) as usize][cb as usize].cost = 61450 as c_float;
                path[(swb + 1) as usize][cb as usize].prev_idx = -1;
                path[(swb + 1) as usize][cb as usize].run = 0;
                cb += 1;
                cb;
            }
        } else {
            let mut minbits: c_float = next_minbits;
            let mut mincb: c_int = next_mincb;
            let mut startcb: c_int = (*sce).band_type[(win * 16 + swb) as usize] as c_int;
            startcb = aac_cb_in_map[startcb as usize] as c_int;
            next_minbits = ::core::f32::INFINITY;
            next_mincb = 0;
            cb = 0;
            while cb < startcb {
                path[(swb + 1) as usize][cb as usize].cost = 61450 as c_float;
                path[(swb + 1) as usize][cb as usize].prev_idx = -1;
                path[(swb + 1) as usize][cb as usize].run = 0;
                cb += 1;
                cb;
            }
            cb = startcb;
            while cb < 15 {
                let mut cost_stay_here_0: c_float = 0.;
                let mut cost_get_here_0: c_float = 0.;
                let mut bits: c_float = 0.0f32;
                if cb >= 12
                    && (*sce).band_type[(win * 16 + swb) as usize] as c_uint
                        != aac_cb_out_map[cb as usize] as c_uint
                {
                    path[(swb + 1) as usize][cb as usize].cost = 61450 as c_float;
                    path[(swb + 1) as usize][cb as usize].prev_idx = -1;
                    path[(swb + 1) as usize][cb as usize].run = 0;
                } else {
                    w = 0;
                    while w < group_len {
                        bits += quantize_band_cost_bits(
                            s,
                            &mut *((*sce).coeffs)
                                .as_mut_ptr()
                                .offset((start + w * 128) as isize),
                            &mut *((*s).scoefs)
                                .as_mut_ptr()
                                .offset((start + w * 128) as isize),
                            size,
                            (*sce).sf_idx[(win * 16 + swb) as usize],
                            aac_cb_out_map[cb as usize] as c_int,
                            0 as c_float,
                            ::core::f32::INFINITY,
                            ptr::null_mut::<c_int>(),
                            ptr::null_mut::<c_float>(),
                        ) as c_float;
                        w += 1;
                        w;
                    }
                    cost_stay_here_0 = path[swb as usize][cb as usize].cost + bits;
                    cost_get_here_0 = minbits + bits + run_bits as c_float + 4 as c_float;
                    if *(run_value_bits[((*sce).ics.num_windows == 8) as c_int as usize])
                        .offset(path[swb as usize][cb as usize].run as isize)
                        as c_int
                        != *(run_value_bits[((*sce).ics.num_windows == 8) as c_int as usize])
                            .offset((path[swb as usize][cb as usize].run + 1) as isize)
                            as c_int
                    {
                        cost_stay_here_0 += run_bits as c_float;
                    }
                    if cost_get_here_0 < cost_stay_here_0 {
                        path[(swb + 1) as usize][cb as usize].prev_idx = mincb;
                        path[(swb + 1) as usize][cb as usize].cost = cost_get_here_0;
                        path[(swb + 1) as usize][cb as usize].run = 1;
                    } else {
                        path[(swb + 1) as usize][cb as usize].prev_idx = cb;
                        path[(swb + 1) as usize][cb as usize].cost = cost_stay_here_0;
                        path[(swb + 1) as usize][cb as usize].run =
                            path[swb as usize][cb as usize].run + 1;
                    }
                    if path[(swb + 1) as usize][cb as usize].cost < next_minbits {
                        next_minbits = path[(swb + 1) as usize][cb as usize].cost;
                        next_mincb = cb;
                    }
                }
                cb += 1;
                cb;
            }
        }
        start += *((*sce).ics.swb_sizes).offset(swb as isize) as c_int;
        swb += 1;
        swb;
    }
    stack_len = 0;
    idx = 0;
    cb = 1;
    while cb < 15 {
        if path[max_sfb as usize][cb as usize].cost < path[max_sfb as usize][idx as usize].cost {
            idx = cb;
        }
        cb += 1;
        cb;
    }
    ppos = max_sfb;
    while ppos > 0 {
        cb = idx;
        stackrun[stack_len as usize] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len as usize] = cb;
        idx =
            path[(ppos - path[ppos as usize][cb as usize].run + 1) as usize][cb as usize].prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
        stack_len;
    }
    start = 0;
    i = stack_len - 1;
    while i >= 0 {
        cb = aac_cb_out_map[stackcb[i as usize] as usize] as c_int;
        put_bits(&mut (*s).pb, 4, cb as BitBuf);
        count = stackrun[i as usize];
        ptr::write_bytes(
            (*sce)
                .zeroes
                .as_mut_ptr()
                .offset((win * 16) as isize)
                .offset(start as isize),
            u8::from(cb == 0),
            count as usize,
        );
        j = 0;
        while j < count {
            (*sce).band_type[(win * 16 + start) as usize] = cb as BandType;
            start += 1;
            start;
            j += 1;
            j;
        }
        while count >= run_esc {
            put_bits(&mut (*s).pb, run_bits, run_esc as BitBuf);
            count -= run_esc;
        }
        put_bits(&mut (*s).pb, run_bits, count as BitBuf);
        i -= 1;
        i;
    }
}
