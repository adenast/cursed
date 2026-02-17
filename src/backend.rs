use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new(&Settings::default())
        .expect("debug-info: cant connect to wayland");

    let (width, height) = enigo
        .main_display()
        .expect("debug-info: cant get screen resolution");

    println!("debug-info: display: {}x{}", width, height);
    println!("debug-info: press Ctrl+C for exit");

    let mut x: f32 = (width / 2) as f32;
    let mut y: f32 = (height / 2) as f32;
    let mut dx: f32 = 6.0;
    let mut dy: f32 = 4.5;

    loop {
        x += dx;
        y += dy;

        if x <= 0.0 || x >= (width - 1) as f32 {
            dx = -dx;
            x = x.clamp(0.0, (width - 1) as f32);
        }
        if y <= 0.0 || y >= (height - 1) as f32 {
            dy = -dy;
            y = y.clamp(0.0, (height - 1) as f32);
        }

        enigo
            .move_mouse(x as i32, y as i32, Coordinate::Abs)
            .expect("debug-info: cant move cursor");

        thread::sleep(Duration::from_millis(16));
    }
}