use colors_transform::{Color, Rgb};
use image;
use plotters::prelude::*;
use plotters_backend;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use screenshots::Image;
use std::io::Cursor;
use std::sync::OnceLock;

const VECTOR_SCOPE_WIDHT: u32 = 250;
const VECTOR_SCOPE_HEIGHT: u32 = 250;
const VECTOR_SCOPE_CENTER: (i32, i32) = (
    (VECTOR_SCOPE_WIDHT / 2) as i32,
    (VECTOR_SCOPE_HEIGHT / 2) as i32,
);
const WAVEFORM_HEIGHT: u32 = 255;
const ANALYZE_SKIP_RATIO: usize = 64;

static VECTOR_SCOPE_BUFFER_SIZE: OnceLock<usize> = OnceLock::new();
#[cold]
fn init_vector_scope_buffer_size() -> usize {
    (VECTOR_SCOPE_WIDHT * VECTOR_SCOPE_HEIGHT * 3) as usize
}

static COLOR_LINE: OnceLock<plotters_backend::BackendColor> = OnceLock::new();
#[cold]
fn init_line_color() -> plotters_backend::BackendColor {
    plotters_backend::BackendColor {
        alpha: 1.0,
        rgb: (100, 100, 100),
    }
}

#[inline(always)]
pub fn draw_vector_scope(image: &Image) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image_vec = image.rgba();
    let mut graph = vec![16; *VECTOR_SCOPE_BUFFER_SIZE.get_or_init(init_vector_scope_buffer_size)];
    {
        let mut root: BitMapBackend<RGBPixel> = BitMapBackend::with_buffer_and_format(
            &mut graph,
            (VECTOR_SCOPE_WIDHT, VECTOR_SCOPE_HEIGHT),
        )
        .unwrap();
        let mut index: usize = 0;
        while index < image_vec.len() {
            let red = image_vec[index];
            let green = image_vec[index + 1];
            let blue = image_vec[index + 2];
            let _alpha = image_vec[index + 3];

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

            index = index + (4 * ANALYZE_SKIP_RATIO);
        }

        // draw center line
        root.draw_line(
            (0, VECTOR_SCOPE_CENTER.1),
            (
                VECTOR_SCOPE_WIDHT.try_into().unwrap(),
                VECTOR_SCOPE_CENTER.1,
            ),
            COLOR_LINE.get_or_init(init_line_color),
        )
        .expect("Error on draw line");
        root.draw_line(
            (VECTOR_SCOPE_CENTER.0, 0),
            (
                VECTOR_SCOPE_CENTER.0,
                VECTOR_SCOPE_HEIGHT.try_into().unwrap(),
            ),
            COLOR_LINE.get_or_init(init_line_color),
        )
        .expect("Error on draw line");

        root.present()?;
    }

    let mut graph_as_image: Vec<u8> = Vec::new();
    image::write_buffer_with_format(
        &mut Cursor::new(&mut graph_as_image),
        &graph,
        VECTOR_SCOPE_WIDHT,
        VECTOR_SCOPE_HEIGHT,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .expect("Failed to write vector scope buffer for create image");
    Ok(graph_as_image)
}

#[inline(always)]
pub fn draw_waveform(image: &Image) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image_vec = image.rgba();
    let image_width = image.width();
    let mut graph = vec![16; (image_width * WAVEFORM_HEIGHT * 3) as usize];
    {
        let mut root: BitMapBackend<RGBPixel> =
            BitMapBackend::with_buffer_and_format(&mut graph, (image_width, WAVEFORM_HEIGHT))
                .unwrap();

        let mut index: usize = 0;
        let mut pixel: usize = 0;
        while index < image_vec.len() {
            let red = image_vec[pixel * 4];
            let green = image_vec[pixel * 4 + 1];
            let blue = image_vec[pixel * 4 + 2];
            // let _alpha = image_vec[pixel * 4 + 3];

            let backend_color_red = plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (red, 0, 0),
            };

            let backend_color_green = plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (0, green, 0),
            };

            let backend_color_blue = plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (0, 0, blue),
            };

            // plot pixels
            let plot_x = (pixel as u32 % image_width) as i32;
            root.draw_pixel((plot_x, red.into()), backend_color_red)
                .expect("Error on plot pixel");
            root.draw_pixel((plot_x, green.into()), backend_color_green)
                .expect("Error on plot pixel");
            root.draw_pixel((plot_x, blue.into()), backend_color_blue)
                .expect("Error on plot pixel");

            // let rgb = Rgb::from(red.into(), green.into(), blue.into());
            // let backend_color = plotters_backend::BackendColor {
            //     alpha: 1.0,
            //     rgb: (red, green, blue),
            // };
            // root.draw_pixel((plot_x, rgb.get_lightness() as i32), backend_color)
            //     .expect("Error on plot pixel");

            pixel += 1;
            index += 4;
        }

        // draw base line
        root.draw_line(
            (0, 255),
            (image_width.try_into().unwrap(), 255),
            COLOR_LINE.get_or_init(init_line_color),
        )
        .expect("Error on draw line");

        // draw limit line
        root.draw_line(
            (0, 0),
            (image_width.try_into().unwrap(), 0),
            COLOR_LINE.get_or_init(init_line_color),
        )
        .expect("Error on draw line");

        root.present()?;
    }

    let mut graph_as_image: Vec<u8> = Vec::new();
    image::write_buffer_with_format(
        &mut Cursor::new(&mut graph_as_image),
        &graph,
        image_width,
        WAVEFORM_HEIGHT,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .expect("Failed to write waveform buffer for create image");
    Ok(graph_as_image)
}
