use std::fs::File;

use symphonia::core::{
    audio::SampleBuffer, codecs::DecoderOptions, errors::Error, formats::FormatOptions,
    io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};

pub fn load_audio_file(path: String) -> Result<Vec<u16>, LoaderError> {
    let Ok(file) = File::open(path) else {
        return Err(LoaderError::FileNotFound);
    };
    let file = Box::new(file);

    let mss = MediaSourceStream::new(file, Default::default());
    let hint = Hint::new();

    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    let mut format = probed.format;
    let track = format.default_track().unwrap();

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .unwrap();

    let track_id = track.id;

    let mut sample_buf = None;
    let mut samples = Vec::new();
    loop {
        let Ok(packet) = format.next_packet() else {
            break;
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                if sample_buf.is_none() {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;

                    sample_buf = Some(SampleBuffer::<u16>::new(duration, spec));
                }

                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    samples.append(&mut buf.samples().to_vec());
                    println!("\rDecoded {} samples", samples.len());
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    return Ok(samples);
}

#[derive(Debug)]
pub enum LoaderError {
    FileNotFound,
}
