use std::collections::HashMap;

use image::{io::Reader as ImageReader, Rgb};

fn main() {
    let colors: HashMap<&str, Rgb<u8>> = [
        ("burgundy", Rgb::from([109, 0, 26])),
        ("dark_red", Rgb::from([190, 0, 57])),
        ("red", Rgb::from([255, 69, 0])),
        ("orange", Rgb::from([255, 168, 0])),
        ("yellow", Rgb::from([255, 214, 53])),
        ("pale_yellow", Rgb::from([255, 248, 184])),
        ("dark_green", Rgb::from([0, 163, 104])),
        ("green", Rgb::from([0, 204, 120])),
        ("light_green", Rgb::from([126, 237, 86])),
        ("dark_teal", Rgb::from([0, 117, 111])),
        ("teal", Rgb::from([0, 158, 170])),
        ("light_teal", Rgb::from([0, 204, 192])),
        ("dark_blue", Rgb::from([36, 80, 164])),
        ("blue", Rgb::from([54, 144, 234])),
        ("light_blue", Rgb::from([81, 233, 244])),
        ("indigo", Rgb::from([73, 58, 193])),
        ("periwinkle", Rgb::from([106, 92, 255])),
        ("lavender", Rgb::from([148, 179, 255])),
        ("dark_purple", Rgb::from([129, 30, 159])),
        ("purple", Rgb::from([180, 74, 192])),
        ("pale_purple", Rgb::from([228, 171, 255])),
        ("magenta", Rgb::from([222, 16, 127])),
        ("pink", Rgb::from([255, 56, 129])),
        ("light_pink", Rgb::from([255, 153, 170])),
        ("dark_brown", Rgb::from([109, 72, 47])),
        ("brown", Rgb::from([156, 105, 38])),
        ("beige", Rgb::from([255, 180, 112])),
        ("black", Rgb::from([0, 0, 0])),
        ("dark_gray", Rgb::from([81, 82, 82])),
        ("gray", Rgb::from([137, 141, 144])),
        ("light_gray", Rgb::from([212, 215, 217])),
        ("white", Rgb::from([255, 255, 255])),
    ]
    .iter()
    .cloned()
    .collect();

    count_pixels(&colors);
}

fn open_img_rgb(name: &str) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img = ImageReader::open(name).unwrap().decode().unwrap();
    img.to_rgb8()
}

fn count_pixels(colors: &HashMap<&str, Rgb<u8>>) {
    let mut values: HashMap<&str, u64> = colors.keys().map(|key| (*key, 0)).collect();

    let img = open_img_rgb("image.png");

    img.pixels().for_each(|pixel| {
        colors.iter().for_each(|(name, color)| {
            if pixel.eq(color) {
                let num = values.get(name).unwrap_or(&0u64);
                values.insert(name, num + 1);
            }
        });
    });

    println!("{:?} -> {}", values, values.values().sum::<u64>());
}
