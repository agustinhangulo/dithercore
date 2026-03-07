use dither::color::Color;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum PalettePresets {
    BW,
    GRAY8,
    APPLE2,
}

pub(crate) const BW: [Color; 2] = [
    Color { r: 0, g: 0, b: 0 },
    Color {
        r: 255,
        g: 255,
        b: 255,
    },
];

// Created in OKLCH color space
// Each color is at intervals of 1/7 (black is 0/7, white is 7/7), giving 2 bounds and 6 intermediate colors
// This isn't rocket science, but it's nice looking
pub(crate) const GRAY_8: [Color; 8] = [
    Color { r: 0, g: 0, b: 0 },
    Color {
        r: 10,
        g: 10,
        b: 10,
    },
    Color {
        r: 42,
        g: 42,
        b: 42,
    },
    Color {
        r: 79,
        g: 79,
        b: 79,
    },
    Color {
        r: 120,
        g: 120,
        b: 120,
    },
    Color {
        r: 163,
        g: 163,
        b: 163,
    },
    Color {
        r: 208,
        g: 208,
        b: 208,
    },
    Color {
        r: 255,
        g: 255,
        b: 255,
    },
];

// 0 - Black
// 0.143 -
// 0.286 -
// 0.429 -
// * Middle - 0.5
// 0.571 -
// 0.714 -
// 0.857 -
// 1 - White

// https://en.wikipedia.org/wiki/Apple_II_graphics#Low-Resolution_(Lo-Res)_graphics
pub(crate) const APPLE2: [Color; 16] = [
    // Black (0)
    Color { r: 0, g: 0, b: 0 },
    // Magenta (1)
    Color {
        r: 255,
        g: 0,
        b: 140,
    },
    // Dark Blue (2)
    Color {
        r: 21,
        g: 16,
        b: 255,
    },
    // Purple (3)
    Color {
        r: 255,
        g: 0,
        b: 255,
    },
    // Dark Green (4)
    Color { r: 0, g: 181, b: 0 },
    // Gray 1 (5)
    Color {
        r: 128,
        g: 128,
        b: 128,
    },
    // Medium Blue (6)
    Color {
        r: 0,
        g: 197,
        b: 255,
    },
    // Light Blue (7)
    Color {
        r: 148,
        g: 143,
        b: 255,
    },
    // Brown (8)
    Color {
        r: 107,
        g: 112,
        b: 0,
    },
    // Orange (9)
    Color {
        r: 255,
        g: 58,
        b: 0,
    },
    // Gray 2 (10)
    Color {
        r: 128,
        g: 128,
        b: 128,
    },
    // Pink (11)
    Color {
        r: 255,
        g: 74,
        b: 255,
    },
    // Light Green (12)
    Color { r: 0, g: 255, b: 0 },
    // Yellow (13)
    Color {
        r: 234,
        g: 239,
        b: 0,
    },
    // Aquamarine (14)
    Color {
        r: 0,
        g: 255,
        b: 115,
    },
    // White (15)
    Color {
        r: 255,
        g: 255,
        b: 255,
    },
];

pub(crate) const STANDARD_8: [Color; 8] = [
    Color { r: 255, g: 0, b: 0 },
    Color {
        r: 255,
        g: 191,
        b: 0,
    },
    Color {
        r: 128,
        g: 255,
        b: 0,
    },
    Color {
        r: 0,
        g: 255,
        b: 64,
    },
    Color {
        r: 0,
        g: 255,
        b: 255,
    },
    Color {
        r: 0,
        g: 64,
        b: 255,
    },
    Color {
        r: 128,
        g: 0,
        b: 255,
    },
    Color {
        r: 255,
        g: 0,
        b: 191,
    },
];
