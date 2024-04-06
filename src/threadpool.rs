use crate::Image;
use crate::MandelbrotSet;
use crate::Setting;
use crate::MAX_THREAD;

/// Thread pool
pub struct ThreadPool;

impl MandelbrotSet for ThreadPool {
    fn calculate(&self, setting: &Setting) -> Image {
        use crate::seq::calc_mandelbrot_set;

        let mut ret = vec![vec![0; setting.img_width as usize]; setting.img_height as usize];
        let iter_max = setting.iteration;
        let real_upper = setting.real_upper;
        let real_lower = setting.real_lower;
        let img_width = setting.img_width;

        let tp = threadpool::ThreadPool::new(MAX_THREAD);

        for (j, row) in ret.iter_mut().enumerate() {
            let y0: f64 = j as f64
                * ((setting.imag_upper - setting.imag_lower) / setting.img_height as f64)
                + setting.imag_lower;
            tp.scope_spawn(move || {
                for (i, v) in row.iter_mut().enumerate() {
                    let x0: f64 =
                        i as f64 * ((real_upper - real_lower) / img_width as f64) + real_lower;
                    let val = calc_mandelbrot_set(y0, x0, iter_max);
                    *v = val;
                }
            });
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use crate::seq::Seq;

    #[test]
    fn test_threadpool_correctness() {
        let setting = Setting {
            output_file: String::new(),
            iteration: 100,
            real_lower: -2.0,
            real_upper: 1.0,
            imag_lower: 1.0,
            imag_upper: -1.0,
            img_width: 1200,
            img_height: 800,
        };

        let start_seq = Instant::now();
        let seq_image = Seq.calculate(&setting);
        let seq_duration = start_seq.elapsed();

        let start_tp = Instant::now();
        let tp_image = ThreadPool.calculate(&setting);
        let tp_duration = start_tp.elapsed();

        println!(
            "Seq time: {:?}, threadpool time: {:?}",
            seq_duration, tp_duration
        );

        for (seq_row, tp_row) in seq_image.into_iter().zip(tp_image) {
            for (seq, tp) in seq_row.into_iter().zip(tp_row) {
                assert_eq!(seq, tp);
            }
        }
    }
}
