pub struct Seq;

// use mandelbrot_set::calc_mandelbrot_set;
use mandelbrot_set::core::calc_mandelbrot_set;
use mandelbrot_set::Image;
use mandelbrot_set::MandelbrotSet;
use mandelbrot_set::Setting;

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
