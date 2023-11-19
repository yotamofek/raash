use crate::{common::*, types::*};
use ::libc;

unsafe fn update_lls(mut m: *mut LLSModel, mut var: *const libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= (*m).indep_count {
        j = i;
        while j <= (*m).indep_count {
            (*m).covariance[i as usize][j as usize] +=
                *var.offset(i as isize) * *var.offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}

pub(crate) unsafe fn avpriv_solve_lls(
    mut m: *mut LLSModel,
    mut threshold: libc::c_double,
    mut min_order: libc::c_ushort,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut factor: *mut [libc::c_double; 36] = &mut *(*((*m).covariance)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
    .as_mut_ptr()
    .offset(0 as libc::c_int as isize)
        as *mut libc::c_double as *mut libc::c_void
        as *mut [libc::c_double; 36];
    let mut covar: *mut [libc::c_double; 36] = &mut *(*((*m).covariance)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
    .as_mut_ptr()
    .offset(1 as libc::c_int as isize)
        as *mut libc::c_double as *mut libc::c_void
        as *mut [libc::c_double; 36];
    let mut covar_y: *mut libc::c_double =
        ((*m).covariance[0 as libc::c_int as usize]).as_mut_ptr();
    let mut count: libc::c_int = (*m).indep_count;
    i = 0 as libc::c_int;
    while i < count {
        j = i;
        while j < count {
            let mut sum: libc::c_double = (*covar.offset(i as isize))[j as usize];
            k = 0 as libc::c_int;
            while k <= i - 1 as libc::c_int {
                sum -= (*factor.offset(i as isize))[k as usize]
                    * (*factor.offset(j as isize))[k as usize];
                k += 1;
                k;
            }
            if i == j {
                if sum < threshold {
                    sum = 1.0f64;
                }
                (*factor.offset(i as isize))[i as usize] = sqrt(sum);
            } else {
                (*factor.offset(j as isize))[i as usize] =
                    sum / (*factor.offset(i as isize))[i as usize];
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < count {
        let mut sum_0: libc::c_double = *covar_y.offset((i + 1 as libc::c_int) as isize);
        k = 0 as libc::c_int;
        while k <= i - 1 as libc::c_int {
            sum_0 -= (*factor.offset(i as isize))[k as usize]
                * (*m).coeff[0 as libc::c_int as usize][k as usize];
            k += 1;
            k;
        }
        (*m).coeff[0 as libc::c_int as usize][i as usize] =
            sum_0 / (*factor.offset(i as isize))[i as usize];
        i += 1;
        i;
    }
    j = count - 1 as libc::c_int;
    while j >= min_order as libc::c_int {
        i = j;
        while i >= 0 as libc::c_int {
            let mut sum_1: libc::c_double = (*m).coeff[0 as libc::c_int as usize][i as usize];
            k = i + 1 as libc::c_int;
            while k <= j {
                sum_1 -=
                    (*factor.offset(k as isize))[i as usize] * (*m).coeff[j as usize][k as usize];
                k += 1;
                k;
            }
            (*m).coeff[j as usize][i as usize] = sum_1 / (*factor.offset(i as isize))[i as usize];
            i -= 1;
            i;
        }
        (*m).variance[j as usize] = *covar_y.offset(0 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i <= j {
            let mut sum_2: libc::c_double = (*m).coeff[j as usize][i as usize]
                * (*covar.offset(i as isize))[i as usize]
                - 2 as libc::c_int as libc::c_double
                    * *covar_y.offset((i + 1 as libc::c_int) as isize);
            k = 0 as libc::c_int;
            while k < i {
                sum_2 += 2 as libc::c_int as libc::c_double
                    * (*m).coeff[j as usize][k as usize]
                    * (*covar.offset(k as isize))[i as usize];
                k += 1;
                k;
            }
            (*m).variance[j as usize] += (*m).coeff[j as usize][i as usize] * sum_2;
            i += 1;
            i;
        }
        j -= 1;
        j;
    }
}
unsafe extern "C" fn evaluate_lls(
    mut m: *mut LLSModel,
    mut param: *const libc::c_double,
    mut order: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut out: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i <= order {
        out += *param.offset(i as isize) * (*m).coeff[order as usize][i as usize];
        i += 1;
        i;
    }
    return out;
}

pub(crate) unsafe fn avpriv_init_lls(mut m: *mut LLSModel, mut indep_count: libc::c_int) {
    (*m) = LLSModel::default();
    (*m).indep_count = indep_count;
    (*m).update_lls = Some(update_lls);
    (*m).evaluate_lls = Some(
        evaluate_lls
            as unsafe extern "C" fn(
                *mut LLSModel,
                *const libc::c_double,
                libc::c_int,
            ) -> libc::c_double,
    );
}
