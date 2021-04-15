use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle, draw_filled_rect, draw_text};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use std::cmp;
use std::fs::create_dir;
use rand::prelude::*;

pub enum IconShape {
    Circle,
    Square,
}

// pub struct Icon {
//     name: &'static str,
//     display_name: &'static str,
//     color: Rgb<u8>,
//     shape: IconShape,
//     size: (i32, i32),
// }

use IconShape::*;

pub fn create(name: &str, color: Rgb<u8>, shape: IconShape, width: i32) {  
    let name = String::from(name);

    let img_w = width * 6 / 5; // the image's width
    let radius = width * 32 / 225;
    let pos = ((img_w - width) / 2, (img_w - width) / 2);

    let img = RgbImage::from_pixel(img_w as u32, img_w as u32, Rgb([48, 50, 61]));
    let mut img = match shape {
        Circle => draw_filled_circle(&img, (img_w / 2, img_w / 2), width / 2, color),
        Square => draw_rounded_rect(&img, (width, width), radius as i32, pos, color),
    };

    let [r, g, b] = color.0;
    let luminance = 0.2126 * srgb_to_lin(r) + 0.7512 * srgb_to_lin(g) + 0.0722 * srgb_to_lin(b);

    let text_color = if luminance > 0.5 {
        darker_rgb(color)
    } else {
        lighter_rgb(color)
    };
    
    img = draw_title(img, text_color, &name);

    let dir_name = match shape {
        Circle => "circle",
        Square => "square",
    };

    match create_dir(dir_name) {
        Ok(_) => {},
        Err(_) => {},
    };
    match img.save_with_format(format!("{}/{}.png", dir_name, name), ImageFormat::Png) {
        Ok(_) => {},
        Err(_) => {},
    };        
}

pub fn create_same_colored_icons(arr: &Vec<&'static str>, color: Rgb<u8>) {
    for app_name in arr {
        create(app_name, color, Circle, 450);
        create(app_name, color, Square, 900);
    }
}

fn srgb_to_lin(color: u8) -> f64 {
    let color: f64 = color as f64 / 255.0;
    if color <= 0.04045 {
        color / 12.92
    } else {
        f64::powf((color + 0.055) / 1.055, 2.4)
    }
}

fn draw_rounded_rect(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    size: (i32, i32),
    radius: i32,
    pos: (i32, i32),
    color: Rgb<u8>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (w, h) = size;
    let (x, y) = pos;
    let r = radius;

    // draw the four circles
    let mut img = draw_filled_circle(img, (x + r, y + r), r, color); // top left
    img = draw_filled_circle(&img, (x + r, y - r + h), r, color); // bottom left
    img = draw_filled_circle(&img, (x - r + w, y - r + h), r, color); // bottom right
    img = draw_filled_circle(&img, (x - r + w, y + r), r, color); // top right

    // draw the rectangle
    let r_w = (w - r * 2) as u32;
    let r_h = (h - r * 2) as u32;

    // rect that is 2 * r less wide than w, with height h
    let mut rect = Rect::at(x + r, y).of_size(r_w, h as u32 + 1);
    img = draw_filled_rect(&img, rect, color);

    // rect that is r wide, and 2 * r shorter than h, drawn on the left side
    rect = Rect::at(x, y + r).of_size(r as u32, r_h);
    img = draw_filled_rect(&img, rect, color);

    // rect that is r wide, and 2 * r shorter than h, drawn on the right side
    rect = Rect::at(x + w - r, y + r).of_size(r as u32 + 1, r_h);
    img = draw_filled_rect(&img, rect, color);

    img
}

fn draw_title(
    mut img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
    text: &str,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let font_data: &[u8] = include_bytes!("now_bold.otf");

    if let Some(font) = Font::try_from_bytes(font_data) {
        let (w, h) = img.dimensions();
        let font_scale = w * 1 / 4;
        let scale = Scale {
            x: font_scale as f32,
            y: font_scale as f32,
        };

        let lines = &text.split_whitespace().collect::<Vec<&str>>();
        let mut i = lines.len() as u32;

        let v_metrics = font.v_metrics(scale);
        let text_max_h = (0.5 + v_metrics.ascent - v_metrics.descent / 2. + v_metrics.line_gap) as u32;

        let mut fit_name = Vec::<String>::new();
        for line in lines {
            let mut line = String::from(*line);
            if line.len() > 6 { line = remove_vowels(line); }
            while line.len() > 6 { line = remove_random_letter(line); }
            fit_name.push(line);
        }

        let calc_text_y = if i <= 1 { single_line_calc_y } else { multiple_lines_calc_y };
        for line in fit_name {
            img = draw_text(
                &mut img,
                color,
                calc_text_x(w, &line, &font, scale),
                calc_text_y(h, text_max_h, i),
                scale,
                &font,
                &line,
            );
            i -= 1;
        }
    }
    img
}

fn lighter_rgb(rgb: Rgb<u8>) -> Rgb<u8> {
    let [r, g, b] = rgb.0;
    Rgb([
        cmp::min(r as u16 + 16, 255) as u8,
        cmp::min(g as u16 + 16, 255) as u8,
        cmp::min(b as u16 + 16, 255) as u8,
    ])
}

fn darker_rgb(rgb: Rgb<u8>) -> Rgb<u8> {
    let [r, g, b] = rgb.0;
    Rgb([
        cmp::max(r as i16 - 24, 0) as u8,
        cmp::max(g as i16 - 24, 0) as u8,
        cmp::max(b as i16 - 24, 0) as u8,
    ])
}

fn calc_text_x(img_w: u32, text: &str, font: &Font, scale: Scale) -> u32 {
    let text_w: f32 = 0.5 + 
        font
            .glyphs_for(text.chars())
            .map(|c| c.scaled(scale).h_metrics().advance_width)
            .sum::<f32>();
    (img_w - text_w as u32) / 2
}

fn remove_vowels(mut text: String) -> String {
    text.retain(|c| !r#"aeiouAEIOU"#.contains(c));
    text
}

fn remove_random_letter(mut text: String) -> String {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(1..text.len());
    text.remove(idx);
    text
}

fn multiple_lines_calc_y(img_h: u32, text_h: u32, i: u32) -> u32 {
    (img_h / 2 + text_h) - text_h * i
}

fn single_line_calc_y(img_h: u32, text_h: u32, _i: u32) -> u32 {
    (img_h - text_h) / 2
}