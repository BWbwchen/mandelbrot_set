#[inline]
pub fn calc_mandelbrot_set(y0: f64, x0: f64, iter_max: i32) -> i32 {
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
