use ffmpeg_src_macro::ffmpeg_src;
use libc::c_double;

#[ffmpeg_src(file = "libavutil/mathematics.c", lines = 217..=225)]
#[inline]
fn eval_poly(coeff: &[c_double], x: c_double) -> c_double {
    coeff
        .iter()
        .rev()
        .copied()
        .reduce(|sum, coeff| (sum * x) + coeff)
        .unwrap()
}

/// 0th order modified bessel function of the first kind.
#[ffmpeg_src(file = "libavutil/mathematics.c", lines = 227..=319, name = "av_bessel_i0")]
pub(crate) fn i0(x: c_double) -> c_double {
    const P1: [c_double; 15] = [
        -2.233_558_263_947_437_5e15,
        -5.505_036_967_301_842_5e14,
        -3.294_008_762_740_775e13,
        -8.492_510_124_711_416e11,
        -1.191_274_610_498_523_7e10,
        -1.031_306_670_873_798_1e8,
        -5.954_562_601_984_789e5,
        -2.412_519_587_604_19e3,
        -7.093_534_744_921_055,
        -1.545_397_779_178_685e-2,
        -2.517_264_467_068_897_6e-5,
        -3.051_722_645_045_107e-8,
        -2.684_344_857_346_848_4e-11,
        -1.598_222_667_565_318_5e-14,
        -5.248_786_662_794_57e-18,
    ];
    const Q1: [c_double; 6] = [
        -2.233_558_263_947_437_5e15,
        7.885_869_256_675_101e12,
        -1.220_706_739_780_897_9e10,
        1.037_708_105_806_216_6e7,
        -4.852_756_017_996_277_5e3,
        1.,
    ];
    const P2: [c_double; 7] = [
        -2.221_026_223_330_657_3e-4,
        1.306_739_203_810_692_4e-2,
        -4.470_080_572_117_445e-1,
        5.567_451_837_124_076,
        -2.351_794_567_923_948e1,
        3.161_132_281_870_113e1,
        -9.609_002_196_865_617,
    ];
    const Q2: [c_double; 8] = [
        -5.519_433_023_100_548e-4,
        3.254_769_759_481_962e-2,
        -1.115_175_918_874_131_3,
        1.398_259_535_389_285_1e1,
        -6.022_800_206_674_334e1,
        8.553_956_325_801_293e1,
        -3.144_669_027_513_549e1,
        1.,
    ];

    if x == 0. {
        return 1.;
    }

    let x = x.abs();

    if x <= 15. {
        let y = x.powi(2);
        eval_poly(&P1, y) / eval_poly(&Q1, y)
    } else {
        let y = x.recip() - 1. / 15.;
        let r = eval_poly(&P2, y) / eval_poly(&Q2, y);
        let factor = x.exp() / x.sqrt();
        factor * r
    }
}
