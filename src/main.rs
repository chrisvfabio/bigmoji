use std::{fs, path::Path};

use image::{imageops, GenericImageView};

fn main() {
    const OUTPUT_DIR: &str = "./output/";

    if Path::new(OUTPUT_DIR).exists() {
        fs::remove_dir_all(OUTPUT_DIR).unwrap();
    }
    fs::create_dir(OUTPUT_DIR).unwrap();

    let input_img = image::open("rust-logo.png").unwrap();

    let output_file_path = Path::new(OUTPUT_DIR);

    let tile_size: u32 = 128 * 2;

    let w = input_img.width() - (input_img.width() % tile_size);
    let h = input_img.height() - (input_img.height() % tile_size);

    let mut norm_image = imageops::resize(&input_img, w, h, imageops::FilterType::Nearest);

    let x_len = norm_image.width();
    let y_len = norm_image.height();

    let mut emoji_output = String::new();

    let mut counter = 0;
    for y in (0..y_len).step_by(tile_size as usize) {
        for x in (0..x_len).step_by(tile_size as usize) {
            counter += 1;
            let new_path = output_file_path.join(format!("rust{}.png", counter));
            imageops::crop(&mut norm_image, x, y, tile_size, tile_size)
                .to_image()
                .save(new_path)
                .unwrap();
            emoji_output += format!(":rust{}:", counter).as_str();
        }
        emoji_output += "\n";
    }
    
    println!("{}", emoji_output);
}
