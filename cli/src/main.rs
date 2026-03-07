use crate::palettes::{APPLE2, BW, GRAY_8, PalettePresets};
use cliclack::{input, intro, outro, select, spinner};
use dither::dither::{DitherMethod, dither};
use image::{self, DynamicImage, GenericImageView, ImageFormat};
use std::path::Path;

mod palettes;

// Given a file path, check if it's a supported file type
fn is_format_supported(path: &Path) -> bool {
    ImageFormat::from_path(path)
        .map(|format| format.can_write())
        .unwrap_or(false)
}

fn main() -> std::io::Result<()> {
    intro("Dither CLI")?;

    let input_path: String = input("Enter the path for your input image.")
        .placeholder("./images/image.jpg")
        .validate(|input: &String| {
            let path = Path::new(input);
            if !path.is_file() {
                return Err("File does not exist, please try again.");
            }

            if !is_format_supported(path) {
                return Err("Input file type not supported. Supported types include common image files such as JPG, PNG, WEBP, TIFF, GIF, etc. Please try again.");
            }

            Ok(())
        })
        .interact()?;

    let output_path: String = input(
        "Enter the path for your output image. (Warning: AVIF encoding can take several minutes)",
    )
    .placeholder("./images/dithered.jpg")
    .validate(|input: &String| {
        // Check if path is a proper path and if its parent exists
        let path = Path::new(input);
        if let Some(parent) = path.parent() {
            if !parent.exists() && !parent.as_os_str().is_empty() {
                return Err("Parent directory of file does not exist, please try again.");
            }
        } else {
            return Err("Output path is not a valid path, please try again.");
        }

        // Check if the output file is a supported format
        if !is_format_supported(path) {
            return Err("Output file type not supported. Supported types include common image files such as JPG, PNG, WEBP, TIFF, GIF, etc. Please try again.");
        }
        Ok(())
    })
    .interact()?;

    // Select color palette
    let palette_option = select("Select a color palette.")
        .item(PalettePresets::BW, "B & W", "Black and white")
        .item(PalettePresets::GRAY8, "Grayscale-8", "8 shades of gray")
        .item(
            PalettePresets::APPLE2,
            "Apple II",
            "Apple II (1977) inspired color palette",
        )
        .interact()?;

    let palette = match palette_option {
        PalettePresets::BW => &BW[..],
        PalettePresets::GRAY8 => &GRAY_8[..],
        PalettePresets::APPLE2 => &APPLE2[..],
    };

    // Select dithering method
    let dither_method = select("Select a dithering method.")
        .item(
            DitherMethod::FloydSteinberg,
            "Floyd-Steinberg",
            "*The* classic error diffusion method",
        )
        .item(
            DitherMethod::Atkinson,
            "Atkinson",
            "An error diffusion method developed at Apple by Bill Atkinson",
        )
        .item(
            DitherMethod::JarvisJudiceNinke,
            "Jarvis-Judice-Ninke",
            "A powerful, complex (but slightly slower) dithering method",
        )
        .item(
            DitherMethod::Stucki,
            "Stucki",
            "An adjusted version of Jarvis-Judice-Ninke",
        )
        .item(
            DitherMethod::Burkes,
            "Burkes",
            "A simpler, less computationally intensive version of Jarvis-Judice-Ninke",
        )
        .item(
            DitherMethod::Sierra,
            "Sierra",
            "Another adjusted version of Jarvis-Judice-Ninke",
        )
        .item(
            DitherMethod::TwoRowSierra,
            "2-Row Sierra",
            "A simplified implementation of Sierra",
        )
        .item(
            DitherMethod::SierraLite,
            "Sierra Lite",
            "An even more simplified implementation of Sierra",
        )
        .item(
            DitherMethod::Bayer2,
            "Bayer 2x2",
            "Bayer dithering with a 2x2 matrix",
        )
        .item(
            DitherMethod::Bayer4,
            "Bayer 4x4",
            "Bayer dithering with a 4x4 matrix",
        )
        .item(
            DitherMethod::Bayer8,
            "Bayer 8x8",
            "Bayer dithering with a 8x8 matrix",
        )
        .item(
            DitherMethod::Threshold,
            "Threshold",
            "All pixels are changed to the closest color in the palette (Technically not dithering)",
        )
        .item(
            DitherMethod::Random,
            "Random",
            "Applies random noise to dither an image",
        )
        .interact()?;

    // let img = image::open(input_path).unwrap();
    let img = image::ImageReader::open(input_path)
        .unwrap()
        .decode()
        .unwrap();

    // Apply image adjustments before dithering (resizing, rotating, etc.)
    // TODO: custom adjustments
    let pre_spinner = spinner();
    pre_spinner.start("Preprocessing image...");
    let img = img.resize(1200, 1200, image::imageops::FilterType::Gaussian);
    pre_spinner.stop("Preprocessing complete.");

    // Dither image
    let dither_spinner = spinner();
    dither_spinner.start("Dithering image...");

    let (width, height) = img.dimensions();
    let mut img_buffer = img.into_rgba8();
    dither(
        &mut img_buffer,
        width as i32,
        height as i32,
        palette,
        dither_method,
    );
    dither_spinner.stop("Dithering complete.");

    // Save image
    let save_spinner = spinner();
    save_spinner.start("Encoding and saving image...");
    let output_image = DynamicImage::ImageRgba8(img_buffer);
    let _save_result = output_image.save(&output_path);
    // let _ = dbg!(_save_result);
    save_spinner.stop("Image saved.");

    let outro_message = format!(
        "Your image is ready at {}. Thank you for using this program.",
        &output_path
    );
    let _ = outro(outro_message);
    Ok(())
}
