use colors_transform::{Color, Rgb};
use image;
use plotters::prelude::*;
use plotters_backend;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use std::io::Cursor;
use std::sync::OnceLock;

const GRAPH_WIDTH: u32 = 250;
const GRAPH_HEIGHT: u32 = 250;
const VECTOR_SCOPE_CENTER: (i32, i32) = ((GRAPH_WIDTH / 2) as i32, (GRAPH_HEIGHT / 2) as i32);
const ANALYZE_SKIP_RATIO: usize = 64;

static BUFFER_SIZE_GRAPH: OnceLock<usize> = OnceLock::new();
static COLOR_LINE: OnceLock<plotters_backend::BackendColor> = OnceLock::new();

#[inline(always)]
pub fn draw_vectorscope(image: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut graph = vec![16; *BUFFER_SIZE_GRAPH.get_or_init(set_graph_size)];
    {
        let mut root: BitMapBackend<RGBPixel> =
            BitMapBackend::with_buffer_and_format(&mut graph, (GRAPH_WIDTH, GRAPH_HEIGHT)).unwrap();
        let mut index: usize = 0;
        while index < image.len() {
            let red = image[index];
            let green = image[index + 1];
            let blue = image[index + 2];
            let _alpha = image[index + 3];

            let rgb = Rgb::from(red.into(), green.into(), blue.into());

            let backend_color = plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (red, green, blue),
            };
            let color_degree: f64 = (rgb.get_hue() + 90.0) as f64;
            let color_degree_as_radians: f64 = color_degree.to_radians();
            let saturation: f64 = rgb.get_saturation() as f64;
            let color_delta_x = saturation * f64::cos(color_degree_as_radians);
            let color_delta_y = saturation * f64::sin(color_degree_as_radians);

            // plot pixels
            root.draw_pixel(
                (
                    (VECTOR_SCOPE_CENTER.0 + color_delta_x as i32),
                    (VECTOR_SCOPE_CENTER.1 - color_delta_y as i32),
                ),
                backend_color,
            )
            .expect("Error on plot pixel");

            // draw center line
            root.draw_line(
                (0, VECTOR_SCOPE_CENTER.1),
                (GRAPH_WIDTH.try_into().unwrap(), VECTOR_SCOPE_CENTER.1),
                COLOR_LINE.get_or_init(set_line_color),
            )
            .expect("Error on draw line");

            root.draw_line(
                (VECTOR_SCOPE_CENTER.0, 0),
                (VECTOR_SCOPE_CENTER.0, GRAPH_HEIGHT.try_into().unwrap()),
                COLOR_LINE.get_or_init(set_line_color),
            )
            .expect("Error on draw line");

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

#[cold]
fn set_graph_size() -> usize {
    (GRAPH_WIDTH * GRAPH_HEIGHT * 3) as usize
}

#[cold]
fn set_line_color() -> plotters_backend::BackendColor {
    plotters_backend::BackendColor {
        alpha: 1.0,
        rgb: (100, 100, 100),
    }
}
