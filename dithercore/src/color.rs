// Ignore alpha channel
#[derive(Debug, PartialEq, Clone)]

/// A color represented as an RGB value.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a new `Color`
    // This is for code brevity, as opposed to any actual need.
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

// Technically computes difference^2 instead of just the difference. But this is simplified
// because we don't care about the actual difference and do care about speed
/// Calculates the square of the Euclidean distance between two colors (e.g. distance<sup>2</sup>).
///
/// This is fine for dithering since algorithms don't need the actual distance and only care about the closest color.
pub(crate) fn color_difference_rgb(color1: &Color, color2: &Color) -> i32 {
    let difference = (color1.r as i32 - color2.r as i32).pow(2)
        + (color1.g as i32 - color2.g as i32).pow(2)
        + (color1.b as i32 - color2.b as i32).pow(2);
    difference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_diff_same_color() {
        let x = Color {
            r: 255,
            g: 255,
            b: 255,
        };
        let y = Color {
            r: 255,
            g: 255,
            b: 255,
        };

        assert_eq!(color_difference_rgb(&x, &y), 0);
    }

    #[test]
    fn color_diff_different_color() {
        let x = Color {
            r: 255,
            g: 255,
            b: 255,
        };
        let y = Color {
            r: 68,
            g: 133,
            b: 136,
        };

        assert_eq!(color_difference_rgb(&x, &y), 64014);
    }
}
