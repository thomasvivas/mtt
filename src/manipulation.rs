use std::{
    path::{Path, PathBuf},
    process::Command,
    ffi::OsStr,
    fs
};

use crate::info;

pub fn extract_frames<P>(video_path: P)
where P: AsRef<Path> + AsRef<OsStr>
{
    if let Ok(entries) = fs::read_dir("./images") {
        for entry in entries {
            if let Ok(entry) = entry {
                fs::remove_file(entry.path()).unwrap();
            }
        }
    }

    Command::new("ffmpeg")
            .arg("-i")
            .arg(video_path)
            .arg("./images/%d.bmp")
            .output().unwrap();
}

// Return the location of the output file
pub fn extract_audio<P>(video_path: P) -> PathBuf
where P: AsRef<Path>
{
    let video_path: PathBuf = video_path.as_ref().to_path_buf();
    let audio_dest = video_path.with_extension("mp3");

    Command::new("ffmpeg")
            .arg("-i")
            .arg(&video_path)
            .arg(&audio_dest)
            .output()
            .expect("Unable to extract audio. Install ffmpeg if not yet done");

    audio_dest
}

// Returns location to the resized file
pub fn resize_image<P>(image_path: P) -> PathBuf 
where P: AsRef<Path>
{
    let final_dimensions: info::Vector2<u32> = info::get_term_dimensions();
    let final_dimensions = format!("scale={}:{}", final_dimensions.x, final_dimensions.y);


    let image_path = image_path.as_ref().to_path_buf();
    let mut output_path = image_path.clone();
    let stem = image_path.file_stem().unwrap().to_str().unwrap();
    output_path.set_file_name(format!("{stem}_resized.bmp"));

    Command::new("ffmpeg")
            .arg("-i")
            .arg(&image_path)
            .arg("-vf")
            .arg(final_dimensions)
            .arg(&output_path)
            .output().unwrap();

    output_path
}
