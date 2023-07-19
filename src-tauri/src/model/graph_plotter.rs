use colors_transform::{Color, Rgb};
use image;
use plotters::prelude::*;
use plotters_backend;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use screenshots::Image;
use std::io::Cursor;
use std::sync::OnceLock;

const BACKGROUND_COLOR: u8 = 16;
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

static AUX_LINE_COLOR: OnceLock<plotters_backend::BackendColor> = OnceLock::new();
#[cold]
fn init_aux_line_color() -> plotters_backend::BackendColor {
    plotters_backend::BackendColor {
        alpha: 1.0,
        rgb: (100, 100, 100),
    }
}

static SKIN_TONE_LINE: OnceLock<(f64, f64)> = OnceLock::new();
#[cold]
fn init_skin_tone_line() -> (f64, f64) {
    let line_x = f64::cos(57f64.to_radians()) * 100.0;
    let line_y = f64::sin(57f64.to_radians()) * 100.0;
    (line_x, line_y)
}

#[inline(always)]
pub fn draw_vector_scope(image: &Image) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image_vec = image.rgba();
    let mut graph = vec![
        BACKGROUND_COLOR;
        *VECTOR_SCOPE_BUFFER_SIZE.get_or_init(init_vector_scope_buffer_size)
    ];
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
            let color_degree: f64 = (rgb.get_hue() + 103.4) as f64;
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

        // draw circle frame
        root.draw_circle(
            VECTOR_SCOPE_CENTER,
            100,
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
            false,
        );

        // draw center line
        root.draw_line(
            (0, VECTOR_SCOPE_CENTER.1),
            (
                VECTOR_SCOPE_WIDHT.try_into().unwrap(),
                VECTOR_SCOPE_CENTER.1,
            ),
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
        )
        .expect("Error on draw line");
        root.draw_line(
            (VECTOR_SCOPE_CENTER.0, 0),
            (
                VECTOR_SCOPE_CENTER.0,
                VECTOR_SCOPE_HEIGHT.try_into().unwrap(),
            ),
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
        )
        .expect("Error on draw line");

        // draw skin tone line
        root.draw_line(
            (VECTOR_SCOPE_CENTER.0, VECTOR_SCOPE_CENTER.1),
            (
                (VECTOR_SCOPE_CENTER.0 as f64 - SKIN_TONE_LINE.get_or_init(init_skin_tone_line).0)
                    as i32,
                (VECTOR_SCOPE_CENTER.1 as f64 - SKIN_TONE_LINE.get_or_init(init_skin_tone_line).1)
                    as i32,
            ),
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
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
    .expect("Failed to write vector scope buffer");
    Ok(graph_as_image)
}

#[inline(always)]
pub fn draw_waveform(image: &Image) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image_vec = image.rgba();
    let image_width = image.width();
    let mut graph = vec![BACKGROUND_COLOR; (image_width * WAVEFORM_HEIGHT * 3) as usize];
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

            pixel += 1;
            index += 4;
        }

        // draw 128 line
        root.draw_line(
            (0, 128),
            (image_width.try_into().unwrap(), 128),
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
        )
        .expect("Error on draw 128 line");

        // draw half widht line
        root.draw_line(
            ((image_width / 2).try_into().unwrap(), 0),
            ((image_width / 2).try_into().unwrap(), 255),
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
        )
        .expect("Error on draw half width line");

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
    .expect("Failed to write waveform buffer");
    Ok(graph_as_image)
}

#[inline(always)]
pub fn draw_waveform_luminance(image: &Image) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image_vec = image.rgba();
    let image_width = image.width();
    let mut graph = vec![BACKGROUND_COLOR; (image_width * WAVEFORM_HEIGHT * 3) as usize];
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

            let plot_x = (pixel as u32 % image_width) as i32;
            let luminace = (0.30 * red as f32) + (0.56 * green as f32) + (0.14 * blue as f32);
            let backend_color = plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (red, green, blue),
            };

            root.draw_pixel((plot_x, luminace as i32), backend_color)
                .expect("Error on plot pixel");

            pixel += 1;
            index += 4;
        }

        // draw 128 line
        root.draw_line(
            (0, 128),
            (image_width.try_into().unwrap(), 128),
            COLOR_LINE.get_or_init(init_line_color),
        )
        .expect("Error on draw 128 line");

        // draw half widht line
        root.draw_line(
            ((image_width / 2).try_into().unwrap(), 0),
            ((image_width / 2).try_into().unwrap(), 255),
            AUX_LINE_COLOR.get_or_init(init_aux_line_color),
        )
        .expect("Error on draw half width line");

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
    .expect("Failed to write waveform buffer");
    Ok(graph_as_image)
}
