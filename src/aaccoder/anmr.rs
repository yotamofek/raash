// note: ANMR is experimental

use std::ptr;

use ffi::codec::AVCodecContext;
use libc::{c_float, c_int, c_uchar};

use super::{
    find_max_val, find_min_book,
    math::{clip_uint8_c, coef2maxsf, coef2minsf},
    quantize_band_cost, trellis,
};
use crate::{
    aacenc::{abs_pow34_v, ctx::AACEncContext},
    aactab::ff_aac_scalefactor_bits,
    common::{av_clip_c, fabsf, log2f, sqrtf},
    types::{FFPsyBand, SingleChannelElement},
};

pub(super) unsafe fn search_for_quantizers(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: c_float,
) {
    let mut q: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut start: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut idx: c_int = 0;
    let mut paths: [[trellis::Path; 61]; 121] = [[trellis::Path { cost: 0., prev: 0 }; 61]; 121];
    let mut bandaddr: [c_int; 121] = [0; 121];
    let mut minq: c_int = 0;
    let mut mincost: c_float = 0.;
    let mut q0f: c_float = 3.402_823_5e38_f32;
    let mut q1f: c_float = 0.0f32;
    let mut qnrgf: c_float = 0.0f32;
    let mut q0: c_int = 0;
    let mut q1: c_int = 0;
    let mut qcnt: c_int = 0 as c_int;
    i = 0 as c_int;
    while i < 1024 as c_int {
        let mut t: c_float = fabsf((*sce).coeffs[i as usize]);
        if t > 0.0f32 {
            q0f = if q0f > t { t } else { q0f };
            q1f = if q1f > t { q1f } else { t };
            qnrgf += t * t;
            qcnt += 1;
            qcnt;
        }
        i += 1;
        i;
    }
    if qcnt == 0 {
        ((*sce).sf_idx).fill(0);
        ((*sce).zeroes).fill(1);
        return;
    }
    q0 = av_clip_c(
        coef2minsf(q0f) as c_int,
        0 as c_int,
        255 as c_int - 1 as c_int,
    );
    q1 = av_clip_c(coef2maxsf(q1f) as c_int, 1 as c_int, 255 as c_int);
    if q1 - q0 > 60 as c_int {
        let mut q0low: c_int = q0;
        let mut q1high: c_int = q1;
        let mut qnrg: c_int = clip_uint8_c(
            (log2f(sqrtf(qnrgf / qcnt as c_float)) * 4 as c_int as c_float - 31 as c_int as c_float
                + 140 as c_int as c_float
                - 36 as c_int as c_float) as c_int,
        ) as c_int;
        q1 = qnrg + 30 as c_int;
        q0 = qnrg - 30 as c_int;
        if q0 < q0low {
            q1 += q0low - q0;
            q0 = q0low;
        } else if q1 > q1high {
            q0 -= q1 - q1high;
            q1 = q1high;
        }
    }
    if q0 == q1 {
        q1 = av_clip_c(q0 + 1 as c_int, 1 as c_int, 255 as c_int);
        q0 = av_clip_c(q1 - 1 as c_int, 0 as c_int, 255 as c_int - 1 as c_int);
    }
    i = 0 as c_int;
    while i < 60 as c_int + 1 as c_int {
        paths[0 as c_int as usize][i as usize].cost = 0.0f32;
        paths[0 as c_int as usize][i as usize].prev = -(1 as c_int);
        i += 1;
        i;
    }
    j = 1 as c_int;
    while j < 121 as c_int {
        i = 0 as c_int;
        while i < 60 as c_int + 1 as c_int {
            paths[j as usize][i as usize].cost = ::core::f32::INFINITY;
            paths[j as usize][i as usize].prev = -(2 as c_int);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    idx = 1 as c_int;
    abs_pow34_v(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as c_int,
    );
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        start = w * 128 as c_int;
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut coefs: *const c_float =
                &mut *((*sce).coeffs).as_mut_ptr().offset(start as isize) as *mut c_float;
            let mut qmin: c_float = 0.;
            let mut qmax: c_float = 0.;
            let mut nz: c_int = 0 as c_int;
            bandaddr[idx as usize] = w * 16 as c_int + g;
            qmin = 2147483647 as c_int as c_float;
            qmax = 0.0f32;
            w2 = 0 as c_int;
            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as c_int + g) as isize)
                        as *mut FFPsyBand;
                if (*band).energy <= (*band).threshold || (*band).threshold == 0.0f32 {
                    (*sce).zeroes[((w + w2) * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                } else {
                    (*sce).zeroes[((w + w2) * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
                    nz = 1 as c_int;
                    i = 0 as c_int;
                    while i < *((*sce).ics.swb_sizes).offset(g as isize) as c_int {
                        let mut t_0: c_float =
                            fabsf(*coefs.offset((w2 * 128 as c_int + i) as isize));
                        if t_0 > 0.0f32 {
                            qmin = if qmin > t_0 { t_0 } else { qmin };
                        }
                        qmax = if qmax > t_0 { qmax } else { t_0 };
                        i += 1;
                        i;
                    }
                }
                w2 += 1;
                w2;
            }
            if nz != 0 {
                let mut minscale: c_int = 0;
                let mut maxscale: c_int = 0;
                let mut minrd: c_float = ::core::f32::INFINITY;
                let mut maxval: c_float = 0.;
                minscale = coef2minsf(qmin) as c_int;
                maxscale = coef2maxsf(qmax) as c_int;
                minscale = av_clip_c(
                    minscale - q0,
                    0 as c_int,
                    60 as c_int + 1 as c_int - 1 as c_int,
                );
                maxscale = av_clip_c(maxscale - q0, 0 as c_int, 60 as c_int + 1 as c_int);
                if minscale == maxscale {
                    maxscale =
                        av_clip_c(minscale + 1 as c_int, 1 as c_int, 60 as c_int + 1 as c_int);
                    minscale = av_clip_c(
                        maxscale - 1 as c_int,
                        0 as c_int,
                        60 as c_int + 1 as c_int - 1 as c_int,
                    );
                }
                maxval = find_max_val(
                    (*sce).ics.group_len[w as usize] as c_int,
                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                    ((*s).scoefs).as_mut_ptr().offset(start as isize),
                );
                q = minscale;
                while q < maxscale {
                    let mut dist: c_float = 0 as c_int as c_float;
                    let mut cb: c_int =
                        find_min_book(maxval, (*sce).sf_idx[(w * 16 as c_int + g) as usize]);
                    w2 = 0 as c_int;
                    while w2 < (*sce).ics.group_len[w as usize] as c_int {
                        let mut band_0: *mut FFPsyBand =
                            &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as c_int + g) as isize)
                                as *mut FFPsyBand;
                        dist += quantize_band_cost(
                            s,
                            coefs.offset((w2 * 128 as c_int) as isize),
                            ((*s).scoefs)
                                .as_mut_ptr()
                                .offset(start as isize)
                                .offset((w2 * 128 as c_int) as isize),
                            *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                            q + q0,
                            cb,
                            lambda / (*band_0).threshold,
                            f32::INFINITY,
                            ptr::null_mut(),
                            ptr::null_mut(),
                        );
                        w2 += 1;
                        w2;
                    }
                    minrd = if minrd > dist { dist } else { minrd };
                    i = 0 as c_int;
                    while i < q1 - q0 {
                        let mut cost: c_float = 0.;
                        cost = paths[(idx - 1 as c_int) as usize][i as usize].cost
                            + dist
                            + ff_aac_scalefactor_bits[(q - i + 60 as c_int) as usize] as c_int
                                as c_float;
                        if cost < paths[idx as usize][q as usize].cost {
                            paths[idx as usize][q as usize].cost = cost;
                            paths[idx as usize][q as usize].prev = i;
                        }
                        i += 1;
                        i;
                    }
                    q += 1;
                    q;
                }
            } else {
                q = 0 as c_int;
                while q < q1 - q0 {
                    paths[idx as usize][q as usize].cost =
                        paths[(idx - 1 as c_int) as usize][q as usize].cost + 1 as c_int as c_float;
                    paths[idx as usize][q as usize].prev = q;
                    q += 1;
                    q;
                }
            }
            (*sce).zeroes[(w * 16 as c_int + g) as usize] = (nz == 0) as c_int as c_uchar;
            start += *((*sce).ics.swb_sizes).offset(g as isize) as c_int;
            idx += 1;
            idx;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    idx -= 1;
    idx;
    mincost = paths[idx as usize][0 as c_int as usize].cost;
    minq = 0 as c_int;
    i = 1 as c_int;
    while i < 60 as c_int + 1 as c_int {
        if paths[idx as usize][i as usize].cost < mincost {
            mincost = paths[idx as usize][i as usize].cost;
            minq = i;
        }
        i += 1;
        i;
    }
    while idx != 0 {
        (*sce).sf_idx[bandaddr[idx as usize] as usize] = minq + q0;
        minq = if paths[idx as usize][minq as usize].prev > 0 as c_int {
            paths[idx as usize][minq as usize].prev
        } else {
            0 as c_int
        };
        idx -= 1;
        idx;
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            w2 = 1 as c_int;
            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                (*sce).sf_idx[((w + w2) * 16 as c_int + g) as usize] =
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                w2 += 1;
                w2;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
