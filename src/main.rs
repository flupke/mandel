mod mouse_state;
mod view;

use minifb::{MouseButton, Window, WindowOptions};
use ndarray::parallel::prelude::*;
use ndarray::{Array1, Array2, Axis};
use num::complex::Complex;

use mouse_state::MouseState;
use view::View;

fn main() {
    let width = 1280;
    let height = 1024;
    let max_iterations = 256;

    let mut window = Window::new(
        "Hello, World!",
        width,
        height,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .expect("failed to create window");
    let mut buffer = Array2::<u32>::zeros((height, width));
    let mut frame = 0;
    let mut now = std::time::Instant::now();
    let mut view = View::new(
        width,
        height,
        Array1::from_vec(vec![0.0, 0.0]),
        Array1::from_vec(vec![4.0, 3.0]),
    );
    let mut mouse_state = MouseState::new();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        mouse_state.set_pos(window.get_mouse_pos(minifb::MouseMode::Pass));
        if window.get_mouse_down(MouseButton::Left) {
            view.pan(&mouse_state.delta);
        }
        if let Some((_dx, dy)) = window.get_scroll_wheel() {
            view.zoom(&mouse_state.pos, 1.0 - dy / 10.0);
        }

        buffer
            .axis_iter_mut(Axis(0))
            .into_par_iter()
            .enumerate()
            .for_each(|(y, mut row)| {
                row.indexed_iter_mut().for_each(|(x, pixel)| {
                    let c = view.map(x, y);
                    let mut z = Complex::new(0.0, 0.0);
                    let mut iteration = 0;
                    while iteration < max_iterations && z.norm() <= 2.0 {
                        z = z * z + c;
                        iteration += 1;
                    }
                    let (r, g, b) = smooth_color_map(z, iteration, max_iterations);
                    *pixel = (r << 16) | (g << 8) | b;
                });
            });

        window
            .update_with_buffer(
                buffer.as_slice().expect("buffer is not contiguous"),
                width,
                height,
            )
            .expect("window update failed");

        frame += 1;
        if frame % 10 == 0 {
            let elapsed = now.elapsed();
            let fps = 10.0 / (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
            window.set_title(&format!("Hello, World! - {:.2} FPS", fps));
            now = std::time::Instant::now();
        }
    }
}

fn smooth_color_map(z: Complex<f64>, iterations: usize, max_iterations: usize) -> (u32, u32, u32) {
    let mu = iterations as f64 + 1.0 - z.norm().log2().log2() / 2.0_f64.log2();
    let normalized_mu = mu / max_iterations as f64;

    let red = (9.0 * (1.0 - normalized_mu) * normalized_mu.powf(3.0) * 255.0) as u32;
    let green = (15.0 * (1.0 - normalized_mu).powf(2.0) * normalized_mu.powf(2.0) * 255.0) as u32;
    let blue = (8.5 * (1.0 - normalized_mu).powf(3.0) * normalized_mu * 255.0) as u32;

    (red, green, blue)
}
