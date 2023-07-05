use clap::Parser;
use termion::cursor;
use rodio::{
    Decoder,
    OutputStream,
    source::Source
};
use std::{
    fs::File,
    io::{
        BufReader,
        Write,
    },
    path::PathBuf,
    path::Path,
    time::{
        Duration,
        Instant
    },
    thread, ffi::OsStr,
};

use indicatif::{
    ProgressBar,
    // ProgressStyle
};

use rayon::prelude::*;
use std::sync::{Mutex, Arc};

mod info;
mod image;
mod manipulation;

#[derive(Parser, Debug)]
struct Args {
    path: PathBuf
}


fn get_frames<P>(video_path: P) -> Vec<String>
where P: AsRef<Path> + AsRef<OsStr>
{
    let frame_count = info::get_frame_count(&video_path) as u64;
    manipulation::extract_frames(&video_path);

    let parse_progress = Arc::new(Mutex::new(ProgressBar::new(frame_count)));

    let res: Vec<String> = (1..=frame_count).into_par_iter().map(|i| {
        let image_path = PathBuf::from(format!("./images/{i}.bmp"));
        let image_path = manipulation::resize_image(&image_path);
        let parsed_image = image::parse_image(&image_path);

        let progress = parse_progress.lock().unwrap();
        progress.inc(1);
        parsed_image
    }).collect();

    parse_progress.lock().unwrap().finish();
    res
}

fn play_audio<P>(audio_path: P) 
where P: AsRef<Path> + AsRef<OsStr>
{
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(&audio_path).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    let audio_duration = Duration::from_secs(info::audio_duration_sec(&audio_path));

    // Yeahh....
    thread::sleep(Duration::from_millis(1200));
    thread::sleep(audio_duration);
}

fn play_video<P>(video_path: P)
where P: AsRef<Path> + AsRef<OsStr>
{
    // Perhaps collect these into a struct relating to video metadata
    let fps = info::get_fps(&video_path);
    let frame_buffer_sec = 1.0 / fps;
    let audio_path = manipulation::extract_audio(&video_path);
    let frames: Vec<String> = get_frames(&video_path);

    thread::spawn(move || play_audio(&audio_path));

    let start_time = Instant::now(); // start timer
    for (i, frame) in frames.iter().enumerate() {
        
        print!("{}", cursor::Goto(1, 1));
        print!("{frame}");
        std::io::stdout().flush().unwrap();

        let frame_time = (i as f64) * frame_buffer_sec; // ideal frame time
        let elapsed = start_time.elapsed().as_secs_f64(); // actual elapsed time

        // error = ideal time - actual time
        let error = frame_time - elapsed;
        let delay = frame_buffer_sec + error; // add error to the ideal frame time
        let delay = delay.clamp(0.0, frame_buffer_sec); // clamp to frame time, should not be negative
        let delay = Duration::from_secs_f64(delay);

        thread::sleep(delay);
    }
}

fn main() {
    let args = Args::parse();
    play_video(&args.path);
}
