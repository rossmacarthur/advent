use std::fmt;

use peter::{Color, Stylize};
use rand::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Pixel {
    value: char,
    color: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct Image(Vec<Vec<Pixel>>);

impl Pixel {
    fn with_color(value: char, color: Color) -> Self {
        Self {
            value,
            color: Some(color),
        }
    }
}

impl Image {
    fn height(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.value, self.color) {
            ('\x00', _) => {
                write!(f, " ")
            }
            (v, Some(color)) => {
                write!(f, "{}", v.fg(color))
            }
            (v, None) => {
                write!(f, "{}", v)
            }
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for pixel in row {
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn pixels(s: &str, color: Color) -> Vec<Pixel> {
    s.chars().map(|c| Pixel::with_color(c, color)).collect()
}

fn center(w: usize, pixels: &[Pixel]) -> Vec<Pixel> {
    assert!(pixels.len() <= w);
    let mut row = vec![Pixel::default(); w];
    let n = (w - pixels.len()) / 2;
    for (i, p) in pixels.iter().copied().enumerate() {
        row[n + i] = p;
    }
    row
}

fn snowflakes(rng: &mut ThreadRng, w: usize, h: usize) -> Image {
    let mut img = vec![vec![Pixel::default(); w]; h];
    let mut i = 0;
    while i < w * h {
        img[i / w][i % w] = Pixel::with_color('·', Color::White);
        i += rng.gen_range(3..6);
    }
    Image(img)
}

fn tree(rng: &mut ThreadRng, mut w: usize) -> Image {
    if w % 2 == 0 {
        w -= 1;
    }

    // Draw the tree
    let mut img = vec![
        center(w, &pixels(" * ", Color::Fixed(229))),
        center(w, &pixels(" /_\\ ", Color::Green)),
        center(w, &pixels(" /_\\_\\ ", Color::Green)),
    ];
    for i in 3..((w - 2) / 2) {
        let left = format!(" /{} ", "_\\".repeat(i));
        let right = format!(" {}\\ ", "/_".repeat(i));
        img.push(center(w, &pixels(&left, Color::Green)));
        img.push(center(w, &pixels(&right, Color::Green)));
    }

    // Add some baubles
    let off = rng.gen_range(0..3);
    for (i, row) in img.iter_mut().skip(2).enumerate() {
        let (left, mid) = row.split_at_mut(w / 3);
        let (mid, right) = mid.split_at_mut(w / 3);

        let mut colors = [Color::Fixed(229), Color::Fixed(69), Color::Fixed(204)];
        colors.rotate_left((off + i) % 3);

        for (section, color) in [left, mid, right].into_iter().zip(colors) {
            if let Some(spot) = section.iter_mut().filter(|p| p.value == '_').choose(rng) {
                *spot = Pixel::with_color('*', color);
            }
        }
    }

    // Finally add the pot
    if w >= 15 {
        let rim = pixels(" [_____] ", Color::Fixed(245));
        img.push(center(w, &rim));
        let bowl = pixels(" \\___/ ", Color::Fixed(245));
        img.push(center(w, &bowl));
    } else {
        let n = if w >= 10 { 3 } else { 1 };
        let bowl = pixels(&format!(" \\{}/ ", "_".repeat(n)), Color::Fixed(245));
        img.push(center(w, &bowl));
    }

    Image(img)
}

fn paste(mut bg: Image, fg: Image, (x, y): (usize, usize)) -> Image {
    let maxy = bg.height() - y;
    for (i, row) in fg.0.into_iter().rev().enumerate() {
        for (j, pixel) in row.into_iter().enumerate() {
            if pixel != Pixel::default() {
                bg.0[maxy - i - 1][x + j] = pixel;
            }
        }
    }
    bg
}

pub fn fun() -> Image {
    let mut rng = rand::thread_rng();
    let width = 46;
    let height = 19;

    // Draw a background of snowflakes
    let mut img = snowflakes(&mut rng, width, height);

    // Generate the first medium sized tree
    let mut x = 0;
    let w = rng.gen_range(11..=17);
    img = paste(img, tree(&mut rng, w), (x, 0));
    x += w;

    // Generate a slightly bigger tree
    let w = rng.gen_range(13..=21);
    img = paste(img, tree(&mut rng, w), (x, 0));
    x += w;

    // If there is enough space for another tree then generate it
    let rem = width - x;
    if rem >= 7 {
        img = paste(img, tree(&mut rng, rem), (x, 0));
    }

    img
}
