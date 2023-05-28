use crate::Image;
use crate::MandelbrotSet;
use crate::Setting;

pub struct SeqSse2;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[inline]
pub(crate) unsafe fn calc_mandelbrot_set_sse2(
    y0: std::arch::x86_64::__m128d,
    x0: std::arch::x86_64::__m128d,
    iter_max: std::arch::x86_64::__m128i,
) -> std::arch::x86_64::__m128i {
    use std::arch::x86_64::*;
    let _zero = _mm_set_pd1(0.0);
    let mut _x: __m128d = _zero;
    let mut _y: __m128d = _zero;

    let mut _repeats: __m128i = _mm_set_epi64x(0, 0);
    let mut _length_squared: __m128d = _zero;

    let mut _add_or_not: __m128i = _mm_set_epi64x(1, 1);

    loop {
        // temp = x * x - y * y + x0
        let _t_x_2 = _mm_mul_pd(_x, _x);
        let _t_y_2 = _mm_mul_pd(_y, _y);
        let _temp = _mm_add_pd(_mm_sub_pd(_t_x_2, _t_y_2), x0);
        // y = 2 * x * y + y0
        let xy = _mm_mul_pd(_mm_set_pd1(2.0), _mm_mul_pd(_x, _y));
        _y = _mm_add_pd(xy, y0);
        // x = temp
        _x = _temp;
        // length_squared = x * x + y * y
        _length_squared = _mm_add_pd(_mm_mul_pd(_x, _x), _mm_mul_pd(_y, _y));

        _repeats = _mm_add_epi64(_repeats, _add_or_not);
        // ************* for condition ****************
        // length_squared < 4
        let mask_lt_4 = _mm_cmplt_pd(_length_squared, _mm_set_pd1(4.0));
        // repeats < iters
        let iters_lt = _mm_cmpgt_epi64(iter_max, _repeats);

        let _condition = _mm_and_si128(_mm_castpd_si128(mask_lt_4), iters_lt);
        let _delta = _mm_and_si128(_mm_set_epi64x(1, 1), _condition);
        _add_or_not = _mm_and_si128(_delta, _add_or_not);
        if _mm_movemask_pd(_mm_castsi128_pd(_condition)) == 0 {
            break;
        }
    }

    _repeats
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
impl MandelbrotSet for SeqSse2 {
    fn calculate(&self, setting: &Setting) -> Image {
        use crate::seq::calc_mandelbrot_set;
        use std::arch::x86_64::*;

        let mut ret = vec![vec![0; setting.img_width as usize]; setting.img_height as usize];
        unsafe {
            let _zero = _mm_set_pd1(0.0);
            let _left = _mm_set_pd1(setting.real_lower);
            let _lower = _mm_set_pd1(setting.imag_lower);
            let xscale =
                _mm_set_pd1((setting.real_upper - setting.real_lower) / setting.img_width as f64);
            let yscale =
                _mm_set_pd1((setting.imag_upper - setting.imag_lower) / setting.img_height as f64);

            let iters_max: __m128i =
                _mm_set_epi64x(setting.iteration as i64, setting.iteration as i64);

            for j in 0..setting.img_height {
                let y0 = _mm_add_pd(_mm_mul_pd(_mm_set_pd1(j as f64), yscale), _lower);
                let mut remain: i32 = 0;
                for i in (0..setting.img_width).step_by(2) {
                    remain = i;
                    let mx: __m128d = _mm_set_pd((i + 1) as f64, (i) as f64);
                    let x0: __m128d = _mm_add_pd(_mm_mul_pd(mx, xscale), _left);

                    let val = calc_mandelbrot_set_sse2(y0, x0, iters_max);

                    // ref: https://github.com/rust-lang/rust/blob/39c03fb65268e3331f381714c664a581a6e86b8c/compiler/rustc_codegen_cranelift/example/std_example.rs#LL181C5-L181C66
                    let (val0, val1) = std::mem::transmute::<_, (i64, i64)>(val);
                    ret[j as usize][i as usize] = val0 as i32;
                    ret[j as usize][i as usize + 1_usize] = val1 as i32;
                }
                for i in remain + 2..setting.img_width {
                    let y0: f64 = j as f64
                        * ((setting.imag_upper - setting.imag_lower) / setting.img_height as f64)
                        + setting.imag_lower;
                    let x0: f64 = i as f64
                        * ((setting.real_upper - setting.real_lower) / setting.img_width as f64)
                        + setting.real_lower;

                    let val = calc_mandelbrot_set(y0, x0, setting.iteration);
                    ret[j as usize][i as usize] = val;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
mod tests {
    use super::*;
    use crate::seq::calc_mandelbrot_set;
    use std::arch::x86_64::*;

    #[test]
    fn test_sse2() {
        let y0_0 = 0.0;
        let x0_0 = 0.0;
        let y0_1 = 1.0;
        let x0_1 = 1.0;
        let iter_max = 10;

        let sse2_output_0: i64;
        let sse2_output_1: i64;
        unsafe {
            let y0_simd = _mm_set_pd(y0_1, y0_0);
            let x0_simd = _mm_set_pd(x0_1, x0_0);

            let iter_max_simd: __m128i = _mm_set_epi64x(iter_max as i64, iter_max as i64);
            let val = calc_mandelbrot_set_sse2(y0_simd, x0_simd, iter_max_simd);
            (sse2_output_0, sse2_output_1) = std::mem::transmute::<_, (i64, i64)>(val);
        }
        assert_eq!(
            sse2_output_0 as i32,
            calc_mandelbrot_set(y0_0, x0_0, iter_max)
        );
        assert_eq!(
            sse2_output_1 as i32,
            calc_mandelbrot_set(y0_1, x0_1, iter_max)
        );
    }
}
