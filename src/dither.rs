// # DITHERING
use crate::color::{Color, color_difference_rgb};
use image::{ImageBuffer, Rgba};

pub enum DitherMethod {
    // Error diffusion dithering
    FloydSteinberg,
    Atkinson,
    JarvisJudiceNinke,
    Stucki,
    Burkes,
    Sierra,
    TwoRowSierra,
    SierraLite,
    // Ordered dithering
    Bayer8,
    Bayer4,
    Bayer2,
    // Misc dithering
    Threshold,
    Random,
}

pub fn find_closest_palette_color(color: Color, palette: &[Color]) -> &Color {
    // A palette isn't a palette if it's empty. A dithering palette should be at least 2 colors.
    if palette.len() < 1 {
        panic!("A color palette cannot be empty.")
    }

    palette
        .iter()
        .min_by_key(|&palette_color| color_difference_rgb(&color, palette_color))
        .unwrap()
}

pub fn dither(
    img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: i32,
    height: i32,
    palette: &[Color],
    dither_method: DitherMethod,
) {
    match dither_method {
        // Error diffusion dithering
        DitherMethod::FloydSteinberg
        | DitherMethod::Atkinson
        | DitherMethod::JarvisJudiceNinke
        | DitherMethod::Stucki
        | DitherMethod::Burkes
        | DitherMethod::Sierra
        | DitherMethod::TwoRowSierra
        | DitherMethod::SierraLite => {
            error_diffusion_dither(img_buffer, width, height, palette, dither_method)
        }
        // Ordered dithering
        DitherMethod::Bayer2 | DitherMethod::Bayer4 | DitherMethod::Bayer8 => {
            bayer_dither(img_buffer, width, height, palette, dither_method);
        }
        // Misc dithering
        DitherMethod::Threshold => todo!(),
        DitherMethod::Random => todo!(),
    }
}

// ## ERROR DIFFUSION DITHERING

// f32 because our error diffusion calculations use floating point math.
pub struct QuantError {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[inline]
fn distribute_error(pixel: &mut Rgba<u8>, err: &QuantError, coeff: f32) {
    pixel[0] = (pixel[0] as f32 + err.r * coeff).round().clamp(0.0, 255.0) as u8;
    pixel[1] = (pixel[1] as f32 + err.g * coeff).round().clamp(0.0, 255.0) as u8;
    pixel[2] = (pixel[2] as f32 + err.b * coeff).round().clamp(0.0, 255.0) as u8;
}

// ### ERROR DIFFUSION KERNELS
// For simplicity, order kernels top -> bottom, left -> right.

struct KernelElement {
    dx: i32,
    dy: i32,
    coefficient: f32,
}

const FLOYD_STEINBERG: [KernelElement; 4] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 7.0 / 16.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 3.0 / 16.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 5.0 / 16.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 1.0 / 16.0,
    },
];

const ATKINSON: [KernelElement; 6] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 1.0 / 8.0,
    },
    KernelElement {
        dx: 2,
        dy: 0,
        coefficient: 1.0 / 8.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 1.0 / 8.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 1.0 / 8.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 1.0 / 8.0,
    },
    KernelElement {
        dx: 0,
        dy: 2,
        coefficient: 1.0 / 8.0,
    },
];

const JARVIS_JUDICE_NINKE: [KernelElement; 12] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 7.0 / 48.0,
    },
    KernelElement {
        dx: 2,
        dy: 0,
        coefficient: 5.0 / 48.0,
    },
    KernelElement {
        dx: -2,
        dy: 1,
        coefficient: 3.0 / 48.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 5.0 / 48.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 7.0 / 48.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 5.0 / 48.0,
    },
    KernelElement {
        dx: 2,
        dy: 1,
        coefficient: 3.0 / 48.0,
    },
    KernelElement {
        dx: -2,
        dy: 2,
        coefficient: 1.0 / 48.0,
    },
    KernelElement {
        dx: -1,
        dy: 2,
        coefficient: 3.0 / 48.0,
    },
    KernelElement {
        dx: 0,
        dy: 2,
        coefficient: 5.0 / 48.0,
    },
    KernelElement {
        dx: 1,
        dy: 2,
        coefficient: 3.0 / 48.0,
    },
    KernelElement {
        dx: 2,
        dy: 2,
        coefficient: 1.0 / 48.0,
    },
];

const STUCKI: [KernelElement; 12] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 8.0 / 42.0,
    },
    KernelElement {
        dx: 2,
        dy: 0,
        coefficient: 4.0 / 42.0,
    },
    KernelElement {
        dx: -2,
        dy: 1,
        coefficient: 2.0 / 42.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 4.0 / 42.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 8.0 / 42.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 4.0 / 42.0,
    },
    KernelElement {
        dx: 2,
        dy: 1,
        coefficient: 2.0 / 42.0,
    },
    KernelElement {
        dx: -2,
        dy: 2,
        coefficient: 1.0 / 42.0,
    },
    KernelElement {
        dx: -1,
        dy: 2,
        coefficient: 2.0 / 42.0,
    },
    KernelElement {
        dx: 0,
        dy: 2,
        coefficient: 4.0 / 42.0,
    },
    KernelElement {
        dx: 1,
        dy: 2,
        coefficient: 2.0 / 42.0,
    },
    KernelElement {
        dx: 2,
        dy: 2,
        coefficient: 1.0 / 42.0,
    },
];

const BURKES: [KernelElement; 7] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 8.0 / 32.0,
    },
    KernelElement {
        dx: 2,
        dy: 0,
        coefficient: 4.0 / 32.0,
    },
    KernelElement {
        dx: -2,
        dy: 1,
        coefficient: 2.0 / 32.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 4.0 / 32.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 8.0 / 32.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 4.0 / 32.0,
    },
    KernelElement {
        dx: 2,
        dy: 1,
        coefficient: 2.0 / 32.0,
    },
];

const SIERRA: [KernelElement; 10] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 5.0 / 32.0,
    },
    KernelElement {
        dx: 2,
        dy: 0,
        coefficient: 3.0 / 32.0,
    },
    KernelElement {
        dx: -2,
        dy: 1,
        coefficient: 2.0 / 32.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 4.0 / 32.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 5.0 / 32.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 4.0 / 32.0,
    },
    KernelElement {
        dx: 2,
        dy: 1,
        coefficient: 2.0 / 32.0,
    },
    KernelElement {
        dx: -1,
        dy: 2,
        coefficient: 2.0 / 32.0,
    },
    KernelElement {
        dx: 0,
        dy: 2,
        coefficient: 3.0 / 32.0,
    },
    KernelElement {
        dx: 1,
        dy: 2,
        coefficient: 2.0 / 32.0,
    },
];

const TWO_ROW_SIERRA: [KernelElement; 7] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 4.0 / 16.0,
    },
    KernelElement {
        dx: 2,
        dy: 0,
        coefficient: 3.0 / 16.0,
    },
    KernelElement {
        dx: -2,
        dy: 1,
        coefficient: 1.0 / 16.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 2.0 / 16.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 3.0 / 16.0,
    },
    KernelElement {
        dx: 1,
        dy: 1,
        coefficient: 2.0 / 16.0,
    },
    KernelElement {
        dx: 2,
        dy: 1,
        coefficient: 1.0 / 16.0,
    },
];

const SIERRA_LITE: [KernelElement; 3] = [
    KernelElement {
        dx: 1,
        dy: 0,
        coefficient: 2.0 / 4.0,
    },
    KernelElement {
        dx: -1,
        dy: 1,
        coefficient: 1.0 / 4.0,
    },
    KernelElement {
        dx: 0,
        dy: 1,
        coefficient: 1.0 / 4.0,
    },
];

fn error_diffusion_dither(
    img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: i32,
    height: i32,
    palette: &[Color],
    dither_method: DitherMethod,
) {
    let kernel = match dither_method {
        DitherMethod::FloydSteinberg => &FLOYD_STEINBERG[..],
        DitherMethod::Atkinson => &ATKINSON[..],
        DitherMethod::JarvisJudiceNinke => &JARVIS_JUDICE_NINKE[..],
        DitherMethod::Stucki => &STUCKI[..],
        DitherMethod::Burkes => &BURKES[..],
        DitherMethod::Sierra => &SIERRA[..],
        DitherMethod::TwoRowSierra => &TWO_ROW_SIERRA[..],
        DitherMethod::SierraLite => &SIERRA_LITE[..],
        _ => return, // If this ever gets here just die I guess?
    };

    for cy in 0..height {
        for cx in 0..width {
            // An Rgba pixel is a tuple (r, g, b, a).
            let curr_pixel = img_buffer.get_pixel_mut(cx as u32, cy as u32);

            let new_color = find_closest_palette_color(
                Color::new(curr_pixel[0], curr_pixel[1], curr_pixel[2]),
                &palette,
            );

            let quant_error = QuantError {
                r: curr_pixel[0] as f32 - new_color.r as f32,
                g: curr_pixel[1] as f32 - new_color.g as f32,
                b: curr_pixel[2] as f32 - new_color.b as f32,
            };

            // Update new pixel
            // *Note: Only update RGB values, alpha channel is preserved as-is
            // May change in the future so that any alpha > 0 is changed  to 255 to prevent colors outside target palette
            curr_pixel[0] = new_color.r;
            curr_pixel[1] = new_color.g;
            curr_pixel[2] = new_color.b;

            for ke in kernel {
                // Bounds checking: Ensure the pixels we are checking actually exist
                if (cx + ke.dx) < 0
                    || (cx + ke.dx) >= width
                    || (cy + ke.dy) < 0
                    || (cy + ke.dy) >= height
                {
                    continue;
                }
                let neighbor = img_buffer.get_pixel_mut((cx + ke.dx) as u32, (cy + ke.dy) as u32);
                distribute_error(neighbor, &quant_error, ke.coefficient);
            }
        }
    }
}

// ## ORDERED DITHERING

// ### BAYER MATRICES
// Stored as 1-dimensional arrays because Rust match needs them to be the same type (roughly)

const BAYER2: [f32; 4] = [0.0, 2.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0];

const BAYER4: [f32; 16] = [
    0.0,
    8.0 / 16.0,
    2.0 / 16.0,
    10.0 / 16.0,
    12.0 / 16.0,
    4.0 / 16.0,
    14.0 / 16.0,
    6.0 / 16.0,
    3.0 / 16.0,
    11.0 / 16.0,
    1.0 / 16.0,
    9.0 / 16.0,
    15.0 / 16.0,
    7.0 / 16.0,
    13.0 / 16.0,
    5.0 / 16.0,
];

const BAYER8: [f32; 64] = [
    0.0,
    32.0 / 64.0,
    8.0 / 64.0,
    40.0 / 64.0,
    2.0 / 64.0,
    34.0 / 64.0,
    10.0 / 64.0,
    42.0 / 64.0,
    48.0 / 64.0,
    16.0 / 64.0,
    56.0 / 64.0,
    24.0 / 64.0,
    50.0 / 64.0,
    18.0 / 64.0,
    58.0 / 64.0,
    26.0 / 64.0,
    12.0 / 64.0,
    44.0 / 64.0,
    4.0 / 64.0,
    36.0 / 64.0,
    14.0 / 64.0,
    46.0 / 64.0,
    6.0 / 64.0,
    38.0 / 64.0,
    60.0 / 64.0,
    28.0 / 64.0,
    52.0 / 64.0,
    20.0 / 64.0,
    62.0 / 64.0,
    30.0 / 64.0,
    54.0 / 64.0,
    22.0 / 64.0,
    3.0 / 64.0,
    35.0 / 64.0,
    11.0 / 64.0,
    43.0 / 64.0,
    1.0 / 64.0,
    33.0 / 64.0,
    9.0 / 64.0,
    41.0 / 64.0,
    51.0 / 64.0,
    19.0 / 64.0,
    59.0 / 64.0,
    27.0 / 64.0,
    49.0 / 64.0,
    17.0 / 64.0,
    57.0 / 64.0,
    25.0 / 64.0,
    15.0 / 64.0,
    47.0 / 64.0,
    7.0 / 64.0,
    39.0 / 64.0,
    13.0 / 64.0,
    45.0 / 64.0,
    5.0 / 64.0,
    37.0 / 64.0,
    63.0 / 64.0,
    31.0 / 64.0,
    55.0 / 64.0,
    23.0 / 64.0,
    61.0 / 64.0,
    29.0 / 64.0,
    53.0 / 64.0,
    21.0 / 64.0,
];

fn bayer_dither(
    img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: i32,
    height: i32,
    palette: &[Color],
    dither_method: DitherMethod,
) {
    let (matrix, matrix_size) = match dither_method {
        DitherMethod::Bayer2 => (&BAYER2[..], 2),
        DitherMethod::Bayer4 => (&BAYER4[..], 4),
        DitherMethod::Bayer8 => (&BAYER8[..], 8),
        _ => return, // If this ever gets here just die I guess?
    };

    // r is color spread; e.g. the distance between colors in your palette. This algorithm assumes colors are evenly spaced.
    // More explained below
    let r = 255.0 / f32::powf(palette.len() as f32, 1.0 / 3.0);

    for cy in 0..height {
        for cx in 0..width {
            let threshold = matrix
                [(cy % matrix_size) as usize * matrix_size as usize + (cx % matrix_size) as usize];

            // An Rgba pixel is a tuple (r, g, b, a).
            let curr_pixel = img_buffer.get_pixel_mut(cx as u32, cy as u32);

            /* Implements algorithm from: https://en.wikipedia.org/wiki/Ordered_dithering#Algorithm
                For each pixel and for each color channel in the pixel:
                    c' = find_closest_palette_color(c + r * (threshold - 0.5))

                Where:
                    c = Current channel value (e.g. pixel[0]), c' = New channel value
                    threshold = Corresponding matrix entry for the pixel
                    r = 255 / cube_root(num_colors); (borrowed this from Dithermark under js/shared/dither-utils.js)

                r is color spread; e.g. the distance between colors in your palette. This algorithm assumes colors are evenly spaced,
                producing inaccurate colors for arbitrary palettes.
                Conceptually, r = 255 / N, where N is the number of bits per channel. We approximate N by using the # of palette colors

                We subtract 1/2 from threshold to normalize to the range [-0.5, 0.5]. This is necessary to keep the image from brightening
            */
            let new_color = find_closest_palette_color(
                Color::new(
                    ((curr_pixel[0] as f32 + r * (threshold - 0.5)).clamp(0.0, 255.0)) as u8,
                    ((curr_pixel[1] as f32 + r * (threshold - 0.5)).clamp(0.0, 255.0)) as u8,
                    ((curr_pixel[2] as f32 + r * (threshold - 0.5)).clamp(0.0, 255.0)) as u8,
                ),
                &palette,
            );

            // Update new pixel
            // *Note: Only update RGB values, alpha channel is preserved as-is
            // May change in the future so that any alpha > 0 is changed  to 255 to prevent colors outside target palette
            curr_pixel[0] = new_color.r;
            curr_pixel[1] = new_color.g;
            curr_pixel[2] = new_color.b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PALETTE: [Color; 8] = [
        Color {
            r: 32,
            g: 26,
            b: 11,
        },
        Color {
            r: 252,
            g: 250,
            b: 226,
        },
        Color {
            r: 73,
            g: 14,
            b: 16,
        },
        Color {
            r: 169,
            g: 34,
            b: 15,
        },
        Color {
            r: 43,
            g: 52,
            b: 124,
        },
        Color {
            r: 244,
            g: 179,
            b: 138,
        },
        Color {
            r: 252,
            g: 231,
            b: 110,
        },
        Color {
            r: 43,
            g: 116,
            b: 9,
        },
    ];

    #[test]
    fn closest_color_same_color_in_palette() {
        let x = Color::new(73, 14, 16);
        let closest_color = find_closest_palette_color(x.clone(), &PALETTE);
        assert_eq!(x, *closest_color);
    }

    #[test]
    fn closest_color_color_not_in_palette() {
        let x = Color::new(253, 251, 227); // PALETTE[1] with each channel + 1
        let closest_color = find_closest_palette_color(x, &PALETTE);
        assert_eq!(Color::new(252, 250, 226), *closest_color);
    }
}
