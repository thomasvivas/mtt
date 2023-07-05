use std::{
    path::Path,
    process::Command,
    ffi::OsStr,
};

pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

// Interpret mathematically "{int}/{int}"
// Look at how beautiful this function is
fn str_div(input: String) -> f64 {
    let constituents: Vec<&str> = input.split('/').collect();
    let a: u64 = constituents[0].trim().parse().unwrap();
    let b: u64 = constituents[1].trim().parse().unwrap();

    a as f64 / b as f64
}

pub fn get_term_dimensions() -> Vector2<u32> {
    const DEFAULT_DIMENSIONS: Vector2<u32> = Vector2 { x:80, y:24 };

    if let Ok((x, y)) = termion::terminal_size() {
        Vector2 { x: x as u32, y: y as u32 }
    } else {
        DEFAULT_DIMENSIONS
    }
}


// Make a commmand builder so you can have to keep on typing arg

pub fn get_frame_count<P>(video_path: P) -> usize
where P: AsRef<Path> + AsRef<OsStr>
{
    let mut command = Command::new("ffprobe");
    command.arg("-v")
           .arg("error")
           .arg("-select_streams")
           .arg("v:0")
           .arg("-count_packets")
           .arg("-show_entries")
           .arg("stream=nb_read_packets")
           .arg("-of")
           .arg("csv=p=0")
           .arg(&video_path);

    let frame_count = command.output().unwrap().stdout;
    let frame_count = String::from_utf8(frame_count).unwrap().trim().parse().unwrap();
    frame_count
}

pub fn audio_duration_sec<P>(audio_path: P) -> u64
where P: AsRef<Path> + AsRef<OsStr>,
{
    let mut command = Command::new("ffprobe");
    command.arg("-i")
           .arg(audio_path)
           .arg("-show_entries")
           .arg("format=duration")
           .arg("-v")
           .arg("quiet")
           .arg("-of")
           .arg("csv=p=0");

    let duration = command.output().unwrap().stdout;
    let duration = String::from_utf8(duration).unwrap();
    let seconds = duration.split('.').collect::<Vec<&str>>()[0];

    seconds.trim().parse().unwrap()
} 

pub fn get_fps<P>(video_path: P) -> f64
where P: AsRef<Path> + AsRef<OsStr>
{
    let mut command = Command::new("ffprobe");
    command.arg("-v")
           .arg("0")
           .arg("-of")
           .arg("csv=p=0")
           .arg("-select_streams")
           .arg("v:0")
           .arg("-show_entries")
           .arg("stream=r_frame_rate")
           .arg(video_path);

    let fps = command.output().unwrap().stdout;
    let fps = String::from_utf8(fps).unwrap();

    str_div(fps)
}
