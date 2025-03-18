mod kernel;
pub use kernel::Shell;

pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let r = (r as f32 / 255.0 * 5.0).round() as u8;
    let g = (g as f32 / 255.0 * 5.0).round() as u8;
    let b = (b as f32 / 255.0 * 5.0).round() as u8;
    16 + 36 * r + 6 * g + b
}
