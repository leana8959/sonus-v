use plotters::{
    prelude::{BitMapBackend, ChartBuilder, Circle, IntoDrawingArea},
    style::{Color, GREEN, WHITE},
};
use stft::{WindowType, STFT};

fn main() {
    let mut reader = claxon::FlacReader::open("audio/field_of_innocence.flac").unwrap();
    let samples: Vec<f64> = reader
        .samples()
        .into_iter()
        .map(|sample| sample.unwrap() as f64)
        .collect();

    let window_type = WindowType::Hanning;
    let window_size = 2048;
    let step_size = 1024;

    let mut stft: STFT<f64> = STFT::new(window_type, window_size, step_size);
    let mut columns: Vec<Vec<f64>> = Vec::new();

    for sample in (samples[..]).chunks(4100) {
        stft.append_samples(sample);

        while stft.contains_enough_to_compute() {
            let mut buffer: Vec<_> = std::iter::repeat(0.).take(stft.output_size()).collect();
            stft.compute_column(&mut buffer);
            columns.push(buffer);
            // println!("{:?}", buffer);
            stft.move_to_next_column();
        }
    }
    println!("{}", columns.len());

    let canvas = BitMapBackend::new("image.png", (1920, 1080)).into_drawing_area();

    canvas.fill(&WHITE).unwrap();

    let mut plot_ctx = ChartBuilder::on(&canvas)
        .build_cartesian_2d(0_f64..27007_f64, 0_f64..2048_f64)
        .unwrap();

    for (x, column) in columns.iter().enumerate() {
        plot_ctx
            .draw_series(
                column
                    .iter()
                    .filter(|x| **x > 2_f64)
                    .enumerate()
                    .map(|(y, amplitude)| Circle::new((x as f64, y as f64), 2, GREEN.stroke_width(*amplitude as u32))),
            )
            .unwrap();
    }

    canvas.present().unwrap();

    ()
}
