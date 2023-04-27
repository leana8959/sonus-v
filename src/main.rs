use plotters::{prelude::*, series::SurfaceSeries};
use stft::{WindowType, STFT};

const WINDOW_SIZE: usize = 8192;
const STEP_SIZE: usize = 4096;

// const IMAGE_WIDTH: u32 = 1920;
// const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_WIDTH: u32 = 600;
const IMAGE_HEIGHT: u32 = 400;

fn main() {
    let mut reader = claxon::FlacReader::open("audio/field_of_innocence.flac").unwrap();
    // let mut reader = claxon::FlacReader::open("audio/blues.flac").unwrap();
    // let mut reader = claxon::FlacReader::open("audio/sine_10mhz.flac").unwrap();
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

    let canvas = BitMapBackend::new("image.png", (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();

    canvas.fill(&WHITE).unwrap();

    let column_count = columns.len();
    let column_width = columns[0].len();

    let mut plot_ctx = ChartBuilder::on(&canvas)
        .build_cartesian_3d(
            0_f64..column_count as f64,
            0_f64..100_f64,
            0_f64..column_width as f64,
        )
        .unwrap();

    plot_ctx.with_projection(|mut p| {
        p.pitch = 0.1;
        // p.yaw = 0.1;
        p.into_matrix()
    });

    plot_ctx.configure_axes().draw().unwrap();

    plot_ctx
        .draw_series(
            SurfaceSeries::xoz(
                (0..column_count).map(|x| x as f64),                  // x axis
                (0..column_width).map(|z| z as f64),                  // z axis
                |x, z| columns[x as usize][z as usize] as f64 * 10.0, // calculate y axis given (x, z)
            )
            .style_func(&|&y| HSLColor(0.07, y / 70.0, 0.7).mix(0.1).filled()),
        )
        .unwrap();

    println!("Done");

    canvas.present().unwrap();
}
