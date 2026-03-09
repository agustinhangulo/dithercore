use crate::palettes::*;
use cliclack::{input, intro, log, outro, select, spinner};
use dithercore::dither::{DitherMethod, dither};
use image::{self, DynamicImage, GenericImageView, ImageFormat};
use std::path::Path;

mod palettes;

/// Given a file path, check if it's a supported file type
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
    let palette_option = select("Select a color palette")
        .item(PalettePresets::BW, "B & W", "Black and white")
        .item(PalettePresets::GRAY8, "Grayscale-8", "8 shades of gray")
        .item(
            PalettePresets::STANDARD8,
            "Standard-8",
            "An 8-color palette of my own creation that should work for dithering most images",
        )
        .item(PalettePresets::STANDARD16, "Standard-16", "A 16-color palette expansion of Standard-8 that should work even better for dithering most images")
        .item(
            PalettePresets::APPLE2,
            "Apple II",
            "Apple II (1977) inspired color palette",
        )
        .item(
            PalettePresets::GAMEBOY,
            "Gameboy",
            "Gameboy (1989) inspired color palette",
        )
        .interact()?;

    let palette = match palette_option {
        PalettePresets::BW => &BW[..],
        PalettePresets::GRAY8 => &GRAY_8[..],
        PalettePresets::STANDARD8 => &STANDARD_8[..],
        PalettePresets::STANDARD16 => &STANDARD_16[..],
        PalettePresets::APPLE2 => &APPLE2[..],
        PalettePresets::GAMEBOY => &GAMEBOY[..],
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

    // let img = image::open(input_path).unwrap();x
    let mut img = image::ImageReader::open(input_path)
        .unwrap()
        .decode()
        .unwrap();

    // Apply image adjustments before dithering (resizing, rotating, etc.)
    let bright_adjust: i32 = input("Brightness: Set an amount of exposure to add/remove.")
        .default_input("0")
        .interact()?;

    let contrast_adjust: f32 = input("Contrast: Set an amount of contrast to add/remove.")
        .default_input("0")
        .interact()?;

    log::info(
        "Resize: To resize your image, set a max width/height. The image is scaled based smaller of the 2 dimensions (aspect ratio is preserved).",
    )?;
    let (width, height) = img.dimensions();
    let new_width: u32 = input("Set a max width to resize your image to (height will change proportionally to maintain aspect ratio). Or just press enter to use original width.").default_input(&format!("{}", width)).interact()?;
    let new_height: u32 = input("Set a max height to resize your image to (width will change proportionally to maintain aspect ratio). Or just press enter to use original height.").default_input(&format!("{}", height)).interact()?;

    let pre_spinner = spinner();
    pre_spinner.start("Preprocessing image...");
    img = img.brighten(bright_adjust);
    img = img.adjust_contrast(contrast_adjust);
    img = img.resize(new_width, new_height, image::imageops::FilterType::Gaussian);
    pre_spinner.stop("Preprocessing complete.");

    // Dither image
    let dither_spinner = spinner();
    dither_spinner.start("Dithering image...");

    let mut img_buffer = img.into_rgba8();
    dither(&mut img_buffer, palette, dither_method);
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
