const MAX_BUFFER_SIZE: usize = 32;

mod all_pass;
mod comb;
mod delay_line;
mod early_reflection_tap_delay_line;
mod low_pass;
mod moorer;

fn interpolate(x1: f32, x2: f32, y1: f32, y2: f32, x: f32) -> f32 {
    let d = x2 - x1;
    if d == 0.0 {
        return y1;
    }
    let dx = (x - x1) / d;
    dx * y2 + (1.0 - dx) * y1
}

pub use moorer::Moorer;
