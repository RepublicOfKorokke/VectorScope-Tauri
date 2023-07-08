use colors_transform::{Color, Rgb};
use image;
use plotters::prelude::*;
use plotters_backend;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use std::io::Cursor;

const GRAPH_WIDTH: u32 = 250;
const GRAPH_HEIGHT: u32 = 250;
const VECTOR_SCOPE_CENTER: (i32, i32) = ((GRAPH_WIDTH / 2) as i32, (GRAPH_HEIGHT / 2) as i32);
const ANALYZE_SKIP_RATIO: usize = 64;

pub fn draw_vectorscope(image: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("image lenght: {}", image.len());
    let mut graph = vec![16; (GRAPH_WIDTH * GRAPH_HEIGHT * 3) as usize];
    {
        let mut root: BitMapBackend<RGBPixel> =
            BitMapBackend::with_buffer_and_format(&mut graph, (GRAPH_WIDTH, GRAPH_HEIGHT)).unwrap();
        let mut index: usize = 0;
        while index < image.len() {
            println!("index {}", index);
            let red = image[index];
            let green = image[index + 1];
            let blue = image[index + 2];
            let _alpha = image[index + 3];

            let rgb = Rgb::from(red.into(), green.into(), blue.into());
            println!("{:?}", rgb);

            let backend_color = plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (red, green, blue),
            };
            let color_degree: f64 = (rgb.get_hue() + 90.0) as f64;
            let color_degree_as_radians: f64 = color_degree.to_radians();
            let saturation: f64 = rgb.get_saturation() as f64;
            let color_delta_x = saturation * f64::cos(color_degree_as_radians);
            let color_delta_y = saturation * f64::sin(color_degree_as_radians);

            root.draw_pixel(
                (
                    (VECTOR_SCOPE_CENTER.0 + color_delta_x as i32),
                    (VECTOR_SCOPE_CENTER.1 - color_delta_y as i32),
                ),
                backend_color,
            )
            .expect("Error on plot pixel");

            index = index + (4 * ANALYZE_SKIP_RATIO);
        }
        root.present()?;
    }

    let mut graph_as_image: Vec<u8> = Vec::new();
    image::write_buffer_with_format(
        &mut Cursor::new(&mut graph_as_image),
        &graph,
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .expect("Failed to write waveform buffer for create image");
    Ok(graph_as_image)
}
