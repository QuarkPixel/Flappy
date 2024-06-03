use bracket_lib::prelude::BTerm;
use crate::palette;

#[allow(dead_code)]
pub fn print(ctx: &mut BTerm, x: i32, y: i32, text: &str) {
    ctx.print_color(x, y, palette::TEXT, palette::TEXT_BG, text);
}

pub fn print_c(ctx: &mut BTerm, y: i32, text: &str) {
    ctx.print_color_centered(y, palette::TEXT, palette::TEXT_BG, text);
}

pub fn blend_colors(color1: (u8, u8, u8), color2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    // 确保t在0.0到1.0之间
    let t = t.clamp(0.0, 1.0);

    let r = (color1.0 as f32 * (1.0 - t) + color2.0 as f32 * t) as u8;
    let g = (color1.1 as f32 * (1.0 - t) + color2.1 as f32 * t) as u8;
    let b = (color1.2 as f32 * (1.0 - t) + color2.2 as f32 * t) as u8;

    (r, g, b)
}