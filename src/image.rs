use std::path::Path;
use image::{
    open,
    DynamicImage
};
use rayon::prelude::*;

 fn luma_to_char(luma_pixel: u8) -> char {
     const CONVERSION_TABLE: &'static str = " .:-=+*#%@";
     const TABLE_LEN: usize = CONVERSION_TABLE.len();

     let lerp_index = (luma_pixel as f64 / 256.0) * (TABLE_LEN as f64); 
     let lerp_index = lerp_index as usize;

     CONVERSION_TABLE.chars().nth(lerp_index).unwrap()
}


pub fn parse_image<P>(image_path: P) -> String
where P: AsRef<Path>
{
    let img = open(image_path)
             .expect("Could not find or open image")
             .into_rgba8();

    // Convert to grayscale
    let img = DynamicImage::ImageRgba8(img)
                           .into_luma8();

    let characters: Vec<char> = img.pixels()
                                   .par_bridge()
                                   .map(|pixel| luma_to_char(pixel[0]))
                                   .collect();

    characters.iter().collect()
}
