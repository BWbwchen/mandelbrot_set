use crate::Image;
use crate::Setting;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use std::fs::File;

pub fn draw_image(image: Image, setting: &Setting) {
    let f = File::create(&setting.output_file).unwrap();
    let png = PngEncoder::new(f);

    let mut buf = vec![0_u8; (setting.img_width * setting.img_height * 3) as usize];

    for (i, v) in image.iter().enumerate() {
        for (j, pixel) in v.iter().enumerate() {
            if *pixel != setting.iteration {
                let start_index = 3 * (i * setting.img_width as usize + j);
                let color = &mut buf[start_index..start_index + 3];
                if pixel & 16 != 0 {
                    color[0] = 240;
                    color[1] = (pixel % 16 * 16) as u8;
                    color[2] = (pixel % 16 * 16) as u8;
                } else {
                    color[0] = (pixel % 16 * 16) as u8;
                }
            }
        }
    }

    png.write_image(
        &buf[..],
        setting.img_width as u32,
        setting.img_height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
