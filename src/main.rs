use std::{cmp::Ordering, env, fs, process::exit};
use symphonia::core::{
    codecs::CODEC_TYPE_NULL, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions,
    probe::Hint,
};

mod audio;
mod misc;

use audio::MonoAudio;
use misc::help;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&1) {
        Ordering::Equal => help(),
        Ordering::Greater => match args[1].as_str() {
            "-h" | "--help" => help(),
            _ => get(&args[1]),
        },
        Ordering::Less => exit(1),
    }
}

fn get(path: &str) {
    let src = fs::File::open(path).expect("failed to open media");

    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("wav");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks")
        .clone();

    let mono = MonoAudio::new(&mut format, &track);

    println!("Sample Rate: {}", mono.sample_rate);
    println!("Total Mono Samples: {}", mono.data.len());
}
