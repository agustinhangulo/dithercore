use dithercore::color::Color;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum PalettePresets {
    BW,
    GRAY8,
    STANDARD8,
    STANDARD16,
    APPLE2,
    GAMEBOY,
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

// Both of my "standard" palettes are very vibes based palettes I made through testing on a handful of images
pub(crate) const STANDARD_8: [Color; 8] = [
    // hsl(24, 4%, 8%) - Warm-leaning off-black
    Color {
        r: 21,
        g: 20,
        b: 20,
    },
    // hsl(24, 16%, 86%) - Warm-leaning off-white
    Color {
        r: 225,
        g: 218,
        b: 214,
    },
    // hsl(0, 56%, 48%) - Red
    Color {
        r: 191,
        g: 54,
        b: 54,
    },
    // hsl(127, 40%, 56%) - Green
    Color {
        r: 98,
        g: 188,
        b: 108,
    },
    // hsl(240, 40%, 56%) - Blue
    Color {
        r: 73,
        g: 73,
        b: 171,
    },
    // hsl(28, 32%, 16%) - Brown
    Color {
        r: 54,
        g: 40,
        b: 28,
    },
    // hsl(28, 51%, 70%) - Skintone
    Color {
        r: 218,
        g: 176,
        b: 140,
    },
    // hsl(56, 64%, 56%) - Yellow
    Color {
        r: 215,
        g: 205,
        b: 71,
    },
];

pub(crate) const STANDARD_16: [Color; 16] = [
    // hsl(24, 4%, 8%) - Warm-leaning off-black
    Color {
        r: 21,
        g: 20,
        b: 20,
    },
    // hsl(24, 16%, 24%) - Warm-leaning dark grey
    Color {
        r: 71,
        g: 59,
        b: 51,
    },
    // hsl(24, 16%, 56%) - Warm-leaning light grey
    Color {
        r: 161,
        g: 139,
        b: 125,
    },
    // hsl(24, 16%, 86%) - Warm-leaning off-white
    Color {
        r: 225,
        g: 218,
        b: 214,
    },
    // hsl(0, 56%, 48%) - Red
    Color {
        r: 191,
        g: 54,
        b: 54,
    },
    // rgb(229, 97, 185) - Pink
    Color {
        r: 229,
        g: 97,
        b: 185,
    },
    // hsl(127, 40%, 56%) - Green
    Color {
        r: 98,
        g: 188,
        b: 108,
    },
    // hsl(72, 80%, 32%) - Green/Yellow (good for plant life imagery)
    Color {
        r: 121,
        g: 147,
        b: 16,
    },
    // hsl(180, 80%, 48%) - Cyan
    Color {
        r: 24,
        g: 220,
        b: 220,
    },
    // hsl(240, 40%, 56%) - Blue
    Color {
        r: 73,
        g: 73,
        b: 171,
    },
    // hsl(240, 100%, 32%) - Deep Blue
    Color { r: 0, g: 0, b: 163 },
    // hsl(240, 40%, 80%) - Light Blue (good for skies)
    Color {
        r: 73,
        g: 73,
        b: 171,
    },
    // hsl(28, 32%, 16%) - Brown
    Color {
        r: 54,
        g: 40,
        b: 28,
    },
    // hsl(28, 51%, 70%) - Skintone
    Color {
        r: 218,
        g: 176,
        b: 140,
    },
    // hsl(56, 64%, 56%) - Yellow (really good in warm higlights)
    Color {
        r: 215,
        g: 205,
        b: 71,
    },
    // hsl(32, 64%, 48%) - Orange (surprisingly really good for skin tones)
    Color {
        r: 201,
        g: 128,
        b: 44,
    },
];

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

// https://en.wikipedia.org/wiki/List_of_video_game_console_palettes#Game_Boy
pub(crate) const GAMEBOY: [Color; 4] = [
    Color {
        r: 41,
        g: 65,
        b: 67,
    },
    Color {
        r: 57,
        g: 89,
        b: 74,
    },
    Color {
        r: 90,
        g: 121,
        b: 66,
    },
    Color {
        r: 123,
        g: 130,
        b: 16,
    },
];
