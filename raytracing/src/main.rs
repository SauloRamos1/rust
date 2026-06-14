use indicatif::ProgressBar;
use std::io;

mod color;
mod vec3;
use color::{Color, write_color};

fn main() -> io::Result<()> {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let progress_bar = ProgressBar::new((image_height * image_width) as u64);
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0 as f32,
            );
            write_color(&mut out, pixel_color);
            progress_bar.inc(1);
        }
    }
    Ok(())
}
