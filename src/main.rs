use plotters::{
    prelude::{BitMapBackend, ChartBuilder, Circle, IntoDrawingArea},
    series::SurfaceSeries,
    style::{Color, HSLColor, GREEN, WHITE},
};
use stft::{WindowType, STFT};

const WINDOW_SIZE: usize = 8192;
const STEP_SIZE: usize = 4096;

fn main() {
    let mut reader = claxon::FlacReader::open("audio/field_of_innocence.flac").unwrap();
    let samples: Vec<f64> = reader
        .samples()
        .into_iter()
        .map(|sample| sample.unwrap() as f64)
        .collect();

    let window_type = WindowType::Hanning;

    let mut stft: STFT<f64> = STFT::new(window_type, WINDOW_SIZE, STEP_SIZE);
    let mut columns: Vec<[f64; WINDOW_SIZE / 2]> = Vec::new();

    for sample in (samples[..]).chunks(5000) {
        stft.append_samples(sample);

        while stft.contains_enough_to_compute() {
            let mut buffer = [0.0; WINDOW_SIZE / 2];
            stft.compute_column(&mut buffer);
            columns.push(buffer);
            // println!("{:?}", buffer);
            stft.move_to_next_column();
        }
    }
    println!("{}", columns.len());

    let canvas = BitMapBackend::new("image.png", (600, 400)).into_drawing_area();

    canvas.fill(&WHITE).unwrap();

    // for pitch in 0..10 {
    let mut plot_ctx = ChartBuilder::on(&canvas)
        .build_cartesian_3d(
            0_f64..columns.len() as f64,
            0_f64..WINDOW_SIZE as f64 * 0.75,
            0_f64..100_f64,
        )
        .unwrap();

    plot_ctx
        .draw_series(
            SurfaceSeries::xoz(
                (0..columns.len()).map(|x| x as f64),
                (0..columns[0].len()).map(|y| y as f64),
                |x, y| columns[x as usize][y as usize] as f64 * 500.0,
            )
            .style_func(&|&v| (&HSLColor(100.0 + v, 1.0, 0.7)).mix(0.8).filled()),
        )
        .unwrap();

    println!("Done");

    canvas.present().unwrap();
    // }
}
