use Vec3 as Point3;
use indicatif::ProgressBar;
use ray::Ray;
use std::io;
use vec3::Vec3;

mod color;
mod ray;
mod vec3;
use color::{Color, write_color};

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    if image_height < 1 {
        image_height = 1;
    }

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    //Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let progress_bar = ProgressBar::new((image_height) as u64);
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        progress_bar.inc(1);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(&mut out, pixel_color)?;
        }
    }
    progress_bar.finish();
    Ok(())
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = center - r.origin();
    let a = Vec3::dot(r.direction(), r.direction());
    let b = -2.0 * Vec3::dot(r.direction(), oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }
    return (-b - discriminant.sqrt()) / (2.0 * a);
}
