# Dithercore - A Rust dithering library

Dithercore is a Rust library for dithering images.

## Interactive CLI Quickstart

To see this library and its CLI in action, build and run the executable using:

```sh
cargo run --release
```

This will open an interactive CLI (built using [cliclack](https://github.com/fadeevab/cliclack)) that allows you to dither images.

## Features

### Supported dithering algorithms

**Error-diffusion algorithms:**
- Floyd-Steinberg
- Atkinson
- Jarvis-Judice-Ninke
- Stucki
- Burkes
- Sierra, Two Row Sierra, Sierra Lite

**Ordered algorithms:**
- Bayer (using matrices of size 2x2, 4x4, and 8x8)

**Miscellaneous algorithms:**
- Threshold
- Random

### Color palettes

This library supports use of arbitrary color palettes (defined as RGB values).

The CLI supports the following presets:
- **B & W:** A black and white palette
- **Grayscale:** A grayscale palette using 8 shades of gray
- **Standard 8:** An 8-color palette that should create decent output on most images
- **Standard 16:** An 16-color palette that should create decent output on most images
- **Gameboy:** A Gameboy inspired palette
- **Apple II:** An Apple II inspired palette

## Library Usage

### Basic dithering example

```rust
use image;
use dithercore::{DitherMethod, dither};
use dithercore::color::Color;

let palette = [ // Basic black and white palette
    Color { r: 0, g: 0, b: 0 },
    Color { r: 255, g: 255, b: 255, },
]

// Open the image
let mut img = image::ImageReader::open("[valid_image_path]")
        .unwrap()
        .decode()
        .unwrap();

// Use image buffer
let mut img_buffer = img.into_rgba8();
dither(&mut img_buffer, palette, DitherMethod::FloydSteinberg);

// Image buffer is now buffered and can be saved, further modified, etc.
```

### 

## Acknowledgements

The architecture for this library was heavily inspired by [tschinz/dithers](https://github.com/tschinz/dithers), another Rust dithering library.

Parts of my algorithm implementations were taken from [allen-garvey/dithermark](https://github.com/allen-garvey/dithermark).

