// use stft::{WindowType, STFT};

fn main() {
    // let mut reader = claxon::FlacReader::open("audio/field_of_innocence.flac").unwrap();
    // let samples: Vec<f64> = reader
    //     .samples()
    //     .into_iter()
    //     .map(|sample| sample.unwrap() as f64)
    //     .collect();
    //
    // let window_type = WindowType::Hanning;
    // let window_size = 1024;
    // let step_size = 512;
    //
    // let mut stft: STFT<f64> = STFT::new(window_type, window_size, step_size);
    //
    // for sample in (samples[..]).chunks(3000) {
    //     stft.append_samples(sample);
    //
    //     while stft.contains_enough_to_compute() {
    //         let mut buffer: Vec<_> = std::iter::repeat(0.).take(stft.output_size()).collect();
    //         stft.compute_column(&mut buffer);
    //         println!("Column {:?}", buffer);
    //         stft.move_to_next_column();
    //     }
    // }
}
