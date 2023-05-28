use crate::Image;
use crate::MandelbrotSet;
use crate::Setting;

pub struct Seq;

#[inline]
fn calc_mandelbrot_set(y0: f64, x0: f64, iter_max: i32) -> i32 {
    let mut repeats = 0;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut length_squared = 0.0;

    while repeats < iter_max && length_squared < 4.0 {
        let temp = x * x - y * y + x0;
        y = 2.0 * x * y + y0;
        x = temp;
        length_squared = x * x + y * y;
        repeats += 1;
    }

    repeats
}

impl MandelbrotSet for Seq {
    fn calculate(&self, setting: &Setting) -> Image {
        let mut ret = vec![vec![0; setting.img_width as usize]; setting.img_height as usize];
        for j in 0..setting.img_height {
            let y0: f64 = j as f64
                * ((setting.imag_upper - setting.imag_lower) / setting.img_height as f64)
                + setting.imag_lower;
            for i in 0..setting.img_width {
                let x0: f64 = i as f64
                    * ((setting.real_upper - setting.real_lower) / setting.img_width as f64)
                    + setting.real_lower;

                let val = calc_mandelbrot_set(y0, x0, setting.iteration);
                ret[j as usize][i as usize] = val;
            }
        }
        ret
    }
}
