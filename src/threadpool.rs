use crate::Image;
use crate::MandelbrotSet;
use crate::Setting;

/// Thread pool
pub struct ThreadPool;

impl MandelbrotSet for ThreadPool {
    fn calculate(&self, setting: &Setting) -> Image {
        use crate::seq::calc_mandelbrot_set;

        let mut ret = vec![vec![0; setting.img_width as usize]; setting.img_height as usize];
        let iter_max = setting.iteration;

        let tp = threadpool::ThreadPool::new(2);
        for j in 0..setting.img_height {
            let y0: f64 = j as f64
                * ((setting.imag_upper - setting.imag_lower) / setting.img_height as f64)
                + setting.imag_lower;
            for i in 0..setting.img_width {
                let x0: f64 = i as f64
                    * ((setting.real_upper - setting.real_lower) / setting.img_width as f64)
                    + setting.real_lower;

                let v = &mut ret[j as usize][i as usize];

                tp.spawn(move || {
                    let val = calc_mandelbrot_set(y0, x0, iter_max);
                    *v = 1;
                    // v = val;
                });
            }
        }
        drop(tp);
        ret
    }
}
