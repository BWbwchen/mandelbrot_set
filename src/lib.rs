pub type Image = Vec<Vec<i32>>;
pub const MAX_THREAD: usize = 2;

pub mod seq;
pub mod seq_sse2;
pub mod threadpool;
pub mod utils;

#[derive(Debug)]
pub struct Setting {
    pub output_file: String,
    pub iteration: i32,
    pub real_lower: f64,
    pub real_upper: f64,
    pub imag_lower: f64,
    pub imag_upper: f64,
    pub img_width: i32,
    pub img_height: i32,
}

pub trait MandelbrotSet {
    fn calculate(&self, setting: &Setting) -> Image;
}
