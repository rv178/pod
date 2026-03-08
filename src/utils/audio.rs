use symphonia::core::{
    audio::{AudioBuffer, Signal},
    codecs::DecoderOptions,
    errors::Error,
    formats::{FormatReader, Track},
};

pub struct MonoAudio {
    pub data: Vec<f32>,
    pub sample_rate: u32,
}

impl MonoAudio {
    pub fn new(format: &mut Box<dyn FormatReader>, track: &Track) -> Self {
        let mut data = Vec::new();
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);

        let dec_opts: DecoderOptions = Default::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        let track_id = track.id;

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let mut f32_buf =
                        AudioBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());

                    decoded.convert(&mut f32_buf);

                    let frames = f32_buf.frames();
                    let channels = f32_buf.spec().channels.count();

                    for frame_index in 0..frames {
                        let mut sum = 0.0;

                        for chan_index in 0..channels {
                            sum += f32_buf.chan(chan_index)[frame_index];
                        }

                        data.push(sum / (channels as f32));
                    }
                }
                Err(Error::IoError(_)) => continue,
                Err(Error::DecodeError(_)) => continue,
                Err(err) => panic!("{}", err),
            }
        }

        Self { data, sample_rate }
    }
}
