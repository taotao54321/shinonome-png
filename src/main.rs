use std::env;
use std::process;

use anyhow::{ensure, Context, Result};
use image::{Rgba, RgbaImage};
use itertools::iproduct;

fn convert(font: bdf::Font) -> Result<RgbaImage> {
    let (width, height) = {
        let glyph = font.glyphs().get(&' ').context("glyph ' ' not found")?;
        (glyph.width(), glyph.height())
    };

    let mut img = RgbaImage::new(16 * width, 6 * height);
    for (i, code) in (0x20..=0x7E).enumerate() {
        let row = i / 16;
        let col = i % 16;
        let y0 = height * row as u32;
        let x0 = width * col as u32;

        let ch = char::from(code);
        let glyph = font
            .glyphs()
            .get(&ch)
            .with_context(|| format!("glyph '{}' not found", ch))?;
        ensure!(
            glyph.width() == width && glyph.height() == height,
            "glyph size mismatch"
        );

        for (dy, dx) in iproduct!(0..height, 0..width) {
            if glyph.get(dx, dy) {
                img.put_pixel(x0 + dx, y0 + dy, Rgba([255, 255, 255, 255]));
            }
        }
    }

    Ok(img)
}

fn usage() -> ! {
    eprintln!("Usage: bdf_to_image <bdf> <image>");
    process::exit(1);
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        usage();
    }
    let path_bdf = &args[1];
    let path_img = &args[2];

    let font = bdf::open(path_bdf)?;

    let img = convert(font)?;
    img.save(path_img)?;

    Ok(())
}
