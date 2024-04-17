//! This crate provides an alternative `izip` macro to work around this r-a bug:
//! <https://github.com/rust-lang/rust-analyzer/issues/11681>

#[macro_export]
macro_rules! izip {
    ($a:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
    };

    ($a:expr, $b:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a).zip($b)
    };

    ($a:expr, $b:expr, $c:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .map(
                #[inline(always)]
                |((a, b), c)| (a, b, c),
            )
    };

    ($a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .zip($d)
            .map(
                #[inline(always)]
                |(((a, b), c), d)| (a, b, c, d),
            )
    };

    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .zip($d)
            .zip($e)
            .map(
                #[inline(always)]
                |((((a, b), c), d), e)| (a, b, c, d, e),
            )
    };

    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .zip($d)
            .zip($e)
            .zip($f)
            .map(
                #[inline(always)]
                |(((((a, b), c), d), e), f)| (a, b, c, d, e, f),
            )
    };

    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .zip($d)
            .zip($e)
            .zip($f)
            .zip($g)
            .map(
                #[inline(always)]
                |((((((a, b), c), d), e), f), g)| (a, b, c, d, e, f, g),
            )
    };

    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .zip($d)
            .zip($e)
            .zip($f)
            .zip($g)
            .zip($h)
            .map(
                #[inline(always)]
                |(((((((a, b), c), d), e), f), g), h)| (a, b, c, d, e, f, g, h),
            )
    };

    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($a)
            .zip($b)
            .zip($c)
            .zip($d)
            .zip($e)
            .zip($f)
            .zip($g)
            .zip($h)
            .zip($i)
            .map(
                #[inline(always)]
                |((((((((a, b), c), d), e), f), g), h), i)| (a, b, c, d, e, f, g, h, i),
            )
    };
}
