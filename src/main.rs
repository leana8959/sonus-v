use raylib::prelude::*;
use stft::{WindowType, STFT};

const WINDOW_SIZE: usize = 2048;
const STEP_SIZE: usize = 1024;
const WIDTH: i32 = 1400;
const HEIGHT: i32 = 900;

// const IMAGE_WIDTH: u32 = 1920;
// const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_WIDTH: u32 = 600;
const IMAGE_HEIGHT: u32 = 400;

fn read() -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut reader = claxon::FlacReader::open("audio/field_of_innocence.flac")?;
    // let mut reader = claxon::FlacReader::open("audio/sine_10mhz.flac").unwrap();
    Ok(reader
        .samples()
        .into_iter()
        .map(|sample| sample.unwrap() as f64)
        .collect())
}

fn create_columns(samples: Vec<f64>) -> Vec<[f64; WINDOW_SIZE / 2]> {
    let window_type = WindowType::Hanning;

    let mut stft: STFT<f64> = STFT::new(window_type, WINDOW_SIZE, STEP_SIZE);
    let mut columns: Vec<[f64; WINDOW_SIZE / 2]> = Vec::new();

    for sample in (samples[..]).chunks(5000) {
        stft.append_samples(sample);

        while stft.contains_enough_to_compute() {
            let mut buffer = [0.0; WINDOW_SIZE / 2];
            stft.compute_column(&mut buffer);
            columns.push(buffer);
            stft.move_to_next_column();
        }
    }

    columns
}

fn main() {
    let samples: Vec<f64> = read().unwrap();
    let columns = create_columns(samples);

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("foo bar").build();
    rl.set_target_fps(1);
    let column_count = columns.len();
    let column_width = columns[0].len();

    let mut filter = 1.0;
    while !rl.window_should_close() {
        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        let pixel_size = Vector2::new(
            WIDTH as f32 / column_count as f32,
            HEIGHT as f32 / column_width as f32,
        );

        println!("filter: {}", filter);
        if filter < 5.0 {
            filter += 0.5;
        } else {
            filter -= 0.5;
        }

        for i in 0..column_count {
            for j in 0..column_width {
                let rect = Rectangle::new(
                    i as f32 * pixel_size.x,
                    j as f32 * pixel_size.y,
                    pixel_size.x,
                    pixel_size.y,
                );
                let amplitude = columns[i][j];
                let color = if amplitude < filter {
                    Color::BLACK
                } else {
                    Color::BEIGE
                };
                draw_handle.draw_rectangle_rec(rect, color);
            }
        }
    }
}
