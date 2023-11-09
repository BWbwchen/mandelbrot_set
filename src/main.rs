use std::env;

#[allow(unused_imports)]
use mandelbrot_set::seq::Seq;
use mandelbrot_set::seq_sse2::SeqSse2;
use mandelbrot_set::utils::draw_image;
use mandelbrot_set::{MandelbrotSet, Setting};

fn choose_method() -> Box<dyn MandelbrotSet> {
    // Box::new(Seq)
    Box::new(SeqSse2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 9 {
        panic!(
            "argument error!
Usage: ./main <output_file> <iteration> <x0> <x1> <y0> <y1> <img_width> <img_height>"
        );
    }

    let setting = Setting {
        output_file: args.get(1).unwrap().to_string(),
        iteration: args.get(2).unwrap().parse().unwrap(),
        real_lower: args.get(3).unwrap().parse().unwrap(),
        real_upper: args.get(4).unwrap().parse().unwrap(),
        imag_lower: args.get(5).unwrap().parse().unwrap(),
        imag_upper: args.get(6).unwrap().parse().unwrap(),
        img_width: args.get(7).unwrap().parse().unwrap(),
        img_height: args.get(8).unwrap().parse().unwrap(),
    };
    dbg!(&setting);

    let image = choose_method().calculate(&setting);
    draw_image(image, &setting);
}
